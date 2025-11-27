#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::MarketState;
use linera_sdk::{
    linera_base_types::{AccountOwner, Amount, TimeDelta},
    abi::WithContractAbi,
    views::{RootView, View},
    Contract, ContractRuntime,
};
use market::{MarketOperation, MarketParameters, MarketResponse};
use oracle_ai_abi::*;

pub struct MarketContract {
    state: MarketState,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(MarketContract);

impl WithContractAbi for MarketContract {
    type Abi = market::MarketAbi;
}

impl Contract for MarketContract {
    type Message = ();
    type Parameters = MarketParameters;
    type InstantiationArgument = ();
    type EventValue = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = MarketState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        MarketContract { state, runtime }
    }

    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {
        // No initialization needed
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        match operation {
            MarketOperation::CreateMarket {
                question,
                description,
                category,
                duration_minutes,
                oracle_mode,
            } => {
                let creator = match self.runtime.authenticated_signer() {
                    Some(signer) => signer,
                    None => return MarketResponse::Error("Must be authenticated".to_string()),
                };

                // Validate duration
                let duration_micros = duration_minutes * 60 * 1_000_000;
                if duration_micros < MIN_MARKET_DURATION_MICROS {
                    return MarketResponse::Error("Duration too short".to_string());
                }
                if duration_micros > MAX_MARKET_DURATION_MICROS {
                    return MarketResponse::Error("Duration too long".to_string());
                }

                let current_time = self.runtime.system_time();
                let closes_at = current_time.saturating_add(TimeDelta::from_micros(duration_micros));

                let market = Market {
                    id: 0, // Will be set by create_market
                    creator,
                    question,
                    description,
                    category,
                    status: MarketStatus::Active,
                    created_at: current_time,
                    closes_at,
                    resolved_at: None,
                    outcome: None,
                    total_pool: Amount::ZERO,
                    up_pool: Amount::ZERO,
                    down_pool: Amount::ZERO,
                    oracle_mode,
                    resolution_source: None,
                };

                match self.state.create_market(market).await {
                    Ok(id) => MarketResponse::MarketId(id),
                    Err(e) => MarketResponse::Error(e),
                }
            }

            MarketOperation::PlaceBet {
                market_id,
                prediction,
                amount,
            } => {
                let bettor = match self.runtime.authenticated_signer() {
                    Some(signer) => signer,
                    None => return MarketResponse::Error("Must be authenticated".to_string()),
                };

                // Validate amount
                if u128::from(amount) < MIN_BET_AMOUNT {
                    return MarketResponse::Error(format!(
                        "Bet too small, minimum is {}",
                        MIN_BET_AMOUNT
                    ));
                }

                // Get and validate market
                let mut market = match self.state.get_market(market_id).await {
                    Ok(m) => m,
                    Err(e) => return MarketResponse::Error(e),
                };

                let current_time = self.runtime.system_time();
                if !market.can_bet(current_time) {
                    return MarketResponse::Error("Market is closed for betting".to_string());
                }

                // Update market pools
                market.total_pool = market.total_pool.saturating_add(amount);
                match prediction {
                    Outcome::Up => market.up_pool = market.up_pool.saturating_add(amount),
                    Outcome::Down => market.down_pool = market.down_pool.saturating_add(amount),
                }

                // Save updated market
                if let Err(e) = self.state.update_market(market).await {
                    return MarketResponse::Error(e);
                }

                // Record bet
                let bet = Bet {
                    market_id,
                    bettor,
                    prediction,
                    amount,
                    placed_at: current_time,
                    claimed: false,
                };

                match self.state.place_bet(bet).await {
                    Ok(_) => MarketResponse::Ok,
                    Err(e) => MarketResponse::Error(e),
                }
            }

            MarketOperation::ResolveMarket { market_id, outcome } => {
                let resolver = match self.runtime.authenticated_signer() {
                    Some(signer) => signer,
                    None => return MarketResponse::Error("Must be authenticated".to_string()),
                };

                let mut market = match self.state.get_market(market_id).await {
                    Ok(m) => m,
                    Err(e) => return MarketResponse::Error(e),
                };

                if market.status != MarketStatus::Active {
                    return MarketResponse::Error("Market already resolved or cancelled".to_string());
                }

                let current_time = self.runtime.system_time();

                market.status = MarketStatus::Resolved;
                market.outcome = Some(outcome);
                market.resolved_at = Some(current_time);

                // Calculate platform fee (5%)
                let fee = Amount::from_attos(u128::from(market.total_pool) * 5 / 100);

                let current_fees = *self.state.platform_fees.get();
                let new_fees = current_fees.saturating_add(fee);
                self.state.platform_fees.set(new_fees);

                match self.state.update_market(market).await {
                    Ok(_) => MarketResponse::Ok,
                    Err(e) => MarketResponse::Error(e),
                }
            }

            MarketOperation::ClaimWinnings { market_id } => {
                let bettor = match self.runtime.authenticated_signer() {
                    Some(signer) => signer,
                    None => return MarketResponse::Error("Must be authenticated".to_string()),
                };

                // Check if already claimed
                if self.state.has_claimed(market_id, &bettor).await {
                    return MarketResponse::Error("Already claimed".to_string());
                }

                let payout = match self.state.calculate_payout(market_id, &bettor).await {
                    Ok(p) => p,
                    Err(e) => return MarketResponse::Error(e),
                };

                if payout.is_zero() {
                    return MarketResponse::Error("No winnings to claim".to_string());
                }

                // Mark as claimed
                if let Err(e) = self.state.mark_claimed(market_id, bettor).await {
                    return MarketResponse::Error(e);
                }

                MarketResponse::Payout(payout)
            }

            MarketOperation::CancelMarket { market_id } => {
                let canceller = match self.runtime.authenticated_signer() {
                    Some(signer) => signer,
                    None => return MarketResponse::Error("Must be authenticated".to_string()),
                };

                let mut market = match self.state.get_market(market_id).await {
                    Ok(m) => m,
                    Err(e) => return MarketResponse::Error(e),
                };

                // Only creator can cancel
                if market.creator != canceller {
                    return MarketResponse::Error("Only creator can cancel".to_string());
                }

                if market.status != MarketStatus::Active {
                    return MarketResponse::Error("Cannot cancel resolved market".to_string());
                }

                market.status = MarketStatus::Cancelled;

                match self.state.update_market(market).await {
                    Ok(_) => MarketResponse::Ok,
                    Err(e) => MarketResponse::Error(e),
                }
            }
        }
    }

    async fn execute_message(&mut self, _message: Self::Message) {
        panic!("Market app does not handle messages");
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}