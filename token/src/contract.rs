#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::TokenState;
use linera_sdk::{
    linera_base_types::{AccountOwner, Amount},
    abi::WithContractAbi,
    views::{RootView, View},
    Contract, ContractRuntime,
};
use token::{DailyBonus, TokenOperation, TokenParameters, TokenResponse};

pub struct TokenContract {
    state: TokenState,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(TokenContract);

impl WithContractAbi for TokenContract {
    type Abi = token::TokenAbi;
}

impl Contract for TokenContract {
    type Message = ();
    type Parameters = TokenParameters;
    type InstantiationArgument = ();
    type EventValue = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = TokenState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        TokenContract { state, runtime }
    }

    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {
        let params = self.runtime.application_parameters();

        // Mint initial supply to all super owners
        for owner in self.runtime.chain_ownership().super_owners.iter() {
            self.state
                .credit(owner, params.initial_supply)
                .await
                .expect("Failed to mint initial supply");
        }
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        match operation {
            TokenOperation::Transfer { to, amount } => {
                let from = self
                    .runtime
                    .authenticated_signer()
                    .expect("Transfer must be authenticated");

                match self.state.transfer(&from, &to, amount).await {
                    Ok(_) => TokenResponse::Ok,
                    Err(e) => TokenResponse::Error(e),
                }
            }

            TokenOperation::ClaimBonus => {
                let owner = self
                    .runtime
                    .authenticated_signer()
                    .expect("ClaimBonus must be authenticated");

                let current_time = self.runtime.system_time();

                // Get or create daily bonus for user
                let mut bonus = self
                    .state
                    .daily_bonuses
                    .get(&owner)
                    .await
                    .unwrap_or(None)
                    .unwrap_or_else(DailyBonus::new);

                let bonus_amount = bonus.claim(current_time);

                if bonus_amount.is_zero() {
                    return TokenResponse::Error("Bonus not ready yet".to_string());
                }

                // Credit the bonus
                self.state
                    .credit(&owner, bonus_amount)
                    .await
                    .expect("Failed to credit bonus");

                // Update bonus record
                self.state
                    .daily_bonuses
                    .insert(&owner, bonus)
                    .expect("Failed to update bonus");

                TokenResponse::Ok
            }

            TokenOperation::Mint { to, amount } => {
                // Only super owners can mint
                let ownership = self.runtime.chain_ownership();
                let caller = self.runtime.authenticated_signer().unwrap();

                if !ownership.super_owners.contains(&caller) {
                    return TokenResponse::Error("Only owner can mint".to_string());
                }

                match self.state.credit(&to, amount).await {
                    Ok(_) => TokenResponse::Ok,
                    Err(e) => TokenResponse::Error(e),
                }
            }
        }
    }

    async fn execute_message(&mut self, _message: Self::Message) {
        panic!("Token app does not handle messages");
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}
