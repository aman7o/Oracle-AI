use async_graphql::{Request, Response};
use linera_sdk::{
    linera_base_types::{AccountOwner, Amount, ApplicationId, Timestamp},
    abi::{ContractAbi, ServiceAbi},
    graphql::GraphQLMutationRoot,
};
use oracle_ai_abi::*;
use serde::{Deserialize, Serialize};

pub struct MarketAbi;

impl ContractAbi for MarketAbi {
    type Operation = MarketOperation;
    type Response = MarketResponse;
}

impl ServiceAbi for MarketAbi {
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Clone, Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum MarketOperation {
    /// Create a new prediction market
    CreateMarket {
        question: String,
        description: String,
        category: MarketCategory,
        duration_minutes: u64,
        oracle_mode: OracleMode,
    },
    /// Place a bet on a market
    PlaceBet {
        market_id: u64,
        prediction: Outcome,
        amount: Amount,
    },
    /// Resolve a market (oracle or creator only)
    ResolveMarket {
        market_id: u64,
        outcome: Outcome,
    },
    /// Claim winnings from a resolved market
    ClaimWinnings {
        market_id: u64,
    },
    /// Cancel a market and refund all bets
    CancelMarket {
        market_id: u64,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum MarketResponse {
    Ok,
    MarketId(u64),
    Payout(Amount),
    Error(String),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MarketParameters {
    pub token_app: ApplicationId<token::TokenAbi>,
}

impl Default for MarketResponse {
    fn default() -> Self {
        MarketResponse::Ok
    }
}
