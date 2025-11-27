use async_graphql::{Request, Response};
use linera_sdk::{
    linera_base_types::{Amount, ApplicationId, Timestamp},
    abi::{ContractAbi, ServiceAbi},
    graphql::GraphQLMutationRoot,
};
use oracle_ai_abi::*;
use serde::{Deserialize, Serialize};

pub struct OracleAbi;

impl ContractAbi for OracleAbi {
    type Operation = OracleOperation;
    type Response = OracleResponse;
}

impl ServiceAbi for OracleAbi {
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Clone, Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum OracleOperation {
    /// Submit a price feed
    SubmitPrice { symbol: String, price: Amount },

    /// AI resolves a market
    ResolveMarketAI {
        market_id: u64,
        outcome: Outcome,
        confidence: f32,
        reasoning: String,
        sources: Vec<String>,
    },

    /// Manual resolution
    ResolveMarketManual { market_id: u64, outcome: Outcome },

    /// Register as an oracle
    RegisterOracle,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum OracleResponse {
    Ok,
    Error(String),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OracleParameters {
    pub market_app: ApplicationId<market::MarketAbi>,
}

impl Default for OracleResponse {
    fn default() -> Self {
        OracleResponse::Ok
    }
}
