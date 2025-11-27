use async_graphql::{Request, Response, SimpleObject};
use linera_sdk::{
    linera_base_types::{AccountOwner, Amount, Timestamp},
    abi::{ContractAbi, ServiceAbi},
    graphql::GraphQLMutationRoot,
};
use oracle_ai_abi::*;
use serde::{Deserialize, Serialize};

/// Token application ABI
pub struct TokenAbi;

impl ContractAbi for TokenAbi {
    type Operation = TokenOperation;
    type Response = TokenResponse;
}

impl ServiceAbi for TokenAbi {
    type Query = Request;
    type QueryResponse = Response;
}

/// Token operations
#[derive(Clone, Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum TokenOperation {
    /// Transfer tokens to another account
    Transfer {
        to: AccountOwner,
        amount: Amount,
    },
    /// Claim daily bonus
    ClaimBonus,
    /// Mint tokens (admin only)
    Mint {
        to: AccountOwner,
        amount: Amount,
    },
}

/// Token responses
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TokenResponse {
    Ok,
    Balance(Amount),
    Error(String),
}

/// Token parameters (set at instantiation)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenParameters {
    pub initial_supply: Amount,
}

/// Daily bonus tracking
#[derive(Clone, Debug, Deserialize, Serialize, SimpleObject)]
pub struct DailyBonus {
    pub last_claim: Timestamp,
    pub amount: Amount,
}

impl DailyBonus {
    pub fn new() -> Self {
        Self {
            last_claim: Timestamp::from(0),
            amount: Amount::from_tokens(100), // 100 tokens
        }
    }

    pub fn can_claim(&self, current_time: Timestamp) -> bool {
        let delta = current_time.delta_since(self.last_claim).as_micros();
        delta >= DAILY_BONUS_COOLDOWN_MICROS
    }

    pub fn claim(&mut self, current_time: Timestamp) -> Amount {
        if self.can_claim(current_time) {
            self.last_claim = current_time;
            self.amount
        } else {
            Amount::ZERO
        }
    }

    pub fn time_until_next_claim(&self, current_time: Timestamp) -> u64 {
        let delta = current_time.delta_since(self.last_claim).as_micros();
        if delta >= DAILY_BONUS_COOLDOWN_MICROS {
            0
        } else {
            DAILY_BONUS_COOLDOWN_MICROS - delta
        }
    }
}
