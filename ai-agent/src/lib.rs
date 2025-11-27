use async_graphql::{Request, Response};
use linera_sdk::{
    linera_base_types::{Amount, ApplicationId},
    abi::{ContractAbi, ServiceAbi},
    graphql::GraphQLMutationRoot,
};
use oracle_ai_abi::*;
use serde::{Deserialize, Serialize};

pub struct AIAgentAbi;

impl ContractAbi for AIAgentAbi {
    type Operation = AIAgentOperation;
    type Response = AIAgentResponse;
}

impl ServiceAbi for AIAgentAbi {
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Clone, Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum AIAgentOperation {
    /// Create a new AI agent
    CreateAgent {
        name: String,
        strategy: AgentStrategy,
    },

    /// Agent places a bet
    PlaceBet {
        agent_id: u64,
        market_id: u64,
        prediction: Outcome,
        amount: Amount,
    },

    /// Update agent stats after market resolution
    UpdateStats {
        agent_id: u64,
        won: bool,
        profit: Amount,
    },

    /// Toggle agent active status
    ToggleAgent {
        agent_id: u64,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AIAgentResponse {
    Ok,
    AgentId(u64),
    Error(String),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AIAgentParameters {
    pub market_app: ApplicationId<market::MarketAbi>,
    pub token_app: ApplicationId<token::TokenAbi>,
}

impl Default for AIAgentResponse {
    fn default() -> Self {
        AIAgentResponse::Ok
    }
}
