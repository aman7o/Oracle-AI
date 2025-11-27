use async_graphql::SimpleObject;
use linera_sdk::{
    linera_base_types::{AccountOwner, Amount},
    views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext},
};
use oracle_ai_abi::{Bet, Market, MarketStatus, Outcome};

#[derive(RootView, SimpleObject)]
#[view(context = ViewStorageContext)]
pub struct MarketState {
    /// All markets
    pub markets: MapView<u64, Market>,

    /// Bets per market
    pub bets: MapView<u64, Vec<Bet>>,

    /// Next market ID
    pub next_market_id: RegisterView<u64>,

    /// Platform fees collected
    pub platform_fees: RegisterView<Amount>,

    /// Claimed winnings tracker
    #[graphql(skip)]
    pub claimed: MapView<(u64, AccountOwner), bool>,
}

impl MarketState {
    /// Create a new market
    pub async fn create_market(&mut self, market: Market) -> Result<u64, String> {
        let id = *self.next_market_id.get();

        let mut market_with_id = market;
        market_with_id.id = id;

        self.markets
            .insert(&id, market_with_id)
            .map_err(|e| format!("Failed to create market: {}", e))?;

        self.next_market_id.set(id + 1);
        Ok(id)
    }

    /// Get a market by ID
    pub async fn get_market(&self, id: u64) -> Result<Market, String> {
        self.markets
            .get(&id)
            .await
            .map_err(|e| format!("Failed to get market: {}", e))?
            .ok_or_else(|| format!("Market {} not found", id))
    }

    /// Update a market
    pub async fn update_market(&mut self, market: Market) -> Result<(), String> {
        self.markets
            .insert(&market.id.clone(), market)
            .map_err(|e| format!("Failed to update market: {}", e))
    }

    /// Place a bet on a market
    pub async fn place_bet(&mut self, bet: Bet) -> Result<(), String> {
        let mut bets = self
            .bets
            .get(&bet.market_id)
            .await
            .map_err(|e| format!("Failed to get bets: {}", e))?
            .unwrap_or_default();

        bets.push(bet.clone());

        self.bets
            .insert(&bet.market_id, bets)
            .map_err(|e| format!("Failed to save bet: {}", e))
    }

    /// Get all bets for a market
    pub async fn get_bets(&self, market_id: u64) -> Result<Vec<Bet>, String> {
        Ok(self
            .bets
            .get(&market_id)
            .await
            .map_err(|e| format!("Failed to get bets: {}", e))?
            .unwrap_or_default())
    }

    /// Calculate payout for a bettor
    pub async fn calculate_payout(
        &self,
        market_id: u64,
        bettor: &AccountOwner,
    ) -> Result<Amount, String> {
        let market = self.get_market(market_id).await?;

        if market.status != MarketStatus::Resolved {
            return Err("Market not resolved yet".to_string());
        }

        let outcome = market
            .outcome
            .ok_or_else(|| "Market has no outcome".to_string())?;

        let bets = self.get_bets(market_id).await?;

        // Find user's winning bet
        let winning_bet = bets
            .iter()
            .find(|b| b.bettor == *bettor && b.prediction == outcome);

        let bet = match winning_bet {
            Some(b) => b,
            None => return Ok(Amount::ZERO),
        };

        let winning_pool = match outcome {
            Outcome::Up => market.up_pool,
            Outcome::Down => market.down_pool,
        };

        if winning_pool.is_zero() {
            return Ok(Amount::ZERO);
        }

        // Payout = (user_bet / winning_pool) * total_pool * 0.95
        // Platform takes 5%
        let total_u128 = u128::from(market.total_pool);
        let bet_u128 = u128::from(bet.amount);
        let winning_u128 = u128::from(winning_pool);

        let payout_u128 = (bet_u128 * total_u128 * 95) / (winning_u128 * 100);

        Ok(Amount::from_attos(payout_u128))
    }

    /// Check if user has claimed
    pub async fn has_claimed(&self, market_id: u64, bettor: &AccountOwner) -> bool {
        self.claimed
            .get(&(market_id, *bettor))
            .await
            .unwrap_or(None)
            .unwrap_or(false)
    }

    /// Mark as claimed
    pub async fn mark_claimed(&mut self, market_id: u64, bettor: AccountOwner) -> Result<(), String> {
        self.claimed
            .insert(&(market_id, bettor), true)
            .map_err(|e| format!("Failed to mark claimed: {}", e))
    }
}