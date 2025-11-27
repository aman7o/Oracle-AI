#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::AIAgentState;
use ai_agent::{AIAgentOperation, AIAgentParameters, AIAgentResponse};
use linera_sdk::{
    linera_base_types::Amount,
    abi::WithContractAbi,
    views::{RootView, View},
    Contract, ContractRuntime,
};
use oracle_ai_abi::*;

pub struct AIAgentContract {
    state: AIAgentState,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(AIAgentContract);

impl WithContractAbi for AIAgentContract {
    type Abi = ai_agent::AIAgentAbi;
}

impl Contract for AIAgentContract {
    type Message = ();
    type Parameters = AIAgentParameters;
    type InstantiationArgument = ();
    type EventValue = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = AIAgentState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        AIAgentContract { state, runtime }
    }

    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {
        // No initialization needed
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        match operation {
            AIAgentOperation::CreateAgent { name, strategy } => {
                let creator = match self.runtime.authenticated_signer() {
                    Some(s) => s,
                    None => return AIAgentResponse::Error("Must be authenticated".to_string()),
                };

                let agent = AIAgent {
                    id: 0, // Will be set by create_agent
                    name,
                    strategy,
                    total_bets: 0,
                    wins: 0,
                    losses: 0,
                    total_profit: Amount::ZERO,
                    accuracy: 0.0,
                    active: true,
                };

                match self.state.create_agent(agent).await {
                    Ok(id) => AIAgentResponse::AgentId(id),
                    Err(e) => AIAgentResponse::Error(e),
                }
            }

            AIAgentOperation::PlaceBet {
                agent_id,
                market_id,
                prediction,
                amount,
            } => {
                // Get agent
                let mut agent = match self.state.get_agent(agent_id).await {
                    Ok(a) => a,
                    Err(e) => return AIAgentResponse::Error(e),
                };

                if !agent.active {
                    return AIAgentResponse::Error("Agent is not active".to_string());
                }

                // Call market app to place bet
                let market_app_id = self.runtime.application_parameters().market_app;
                let call = market::MarketOperation::PlaceBet { 
                    market_id, 
                    prediction, 
                    amount 
                };

                self.runtime.call_application(true, market_app_id, &call);

                // Update agent stats
                agent.total_bets += 1;

                if let Err(e) = self.state.update_agent(agent).await {
                    return AIAgentResponse::Error(e);
                }

                AIAgentResponse::Ok
            }

            AIAgentOperation::UpdateStats {
                agent_id,
                won,
                profit,
            } => {
                let mut agent = match self.state.get_agent(agent_id).await {
                    Ok(a) => a,
                    Err(e) => return AIAgentResponse::Error(e),
                };

                agent.update_stats(won, profit);

                match self.state.update_agent(agent).await {
                    Ok(_) => AIAgentResponse::Ok,
                    Err(e) => AIAgentResponse::Error(e),
                }
            }

            AIAgentOperation::ToggleAgent { agent_id } => {
                let mut agent = match self.state.get_agent(agent_id).await {
                    Ok(a) => a,
                    Err(e) => return AIAgentResponse::Error(e),
                };

                agent.active = !agent.active;

                match self.state.update_agent(agent).await {
                    Ok(_) => AIAgentResponse::Ok,
                    Err(e) => AIAgentResponse::Error(e),
                }
            }
        }
    }

    async fn execute_message(&mut self, _message: Self::Message) {
        panic!("AI-Agent app does not handle messages");
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}
