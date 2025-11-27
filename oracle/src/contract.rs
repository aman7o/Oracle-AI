#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::OracleState;
use linera_sdk::{
    abi::WithContractAbi,
    views::{RootView, View},
    Contract, ContractRuntime,
};
use oracle::{OracleOperation, OracleParameters, OracleResponse};
use oracle_ai_abi::*;

pub struct OracleContract {
    state: OracleState,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(OracleContract);

impl WithContractAbi for OracleContract {
    type Abi = oracle::OracleAbi;
}

impl Contract for OracleContract {
    type Message = ();
    type Parameters = OracleParameters;
    type InstantiationArgument = ();
    type EventValue = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = OracleState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        OracleContract { state, runtime }
    }

    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {
        // No initialization needed
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        match operation {
            OracleOperation::SubmitPrice { symbol, price } => {
                let submitter = match self.runtime.authenticated_signer() {
                    Some(s) => s,
                    None => return OracleResponse::Error("Must be authenticated".to_string()),
                };

                // Check if submitter is registered oracle
                if !self.state.is_oracle(&submitter).await {
                    return OracleResponse::Error("Not a registered oracle".to_string());
                }

                let timestamp = self.runtime.system_time().micros();

                match self.state.submit_price(symbol, price, timestamp).await {
                    Ok(_) => OracleResponse::Ok,
                    Err(e) => OracleResponse::Error(e),
                }
            }

            OracleOperation::ResolveMarketAI {
                market_id,
                outcome,
                confidence,
                reasoning,
                sources,
            } => {
                let resolver = match self.runtime.authenticated_signer() {
                    Some(s) => s,
                    None => return OracleResponse::Error("Must be authenticated".to_string()),
                };

                // Check if resolver is registered oracle
                if !self.state.is_oracle(&resolver).await {
                    return OracleResponse::Error("Not a registered oracle".to_string());
                }

                let resolved_at = self.runtime.system_time();

                let resolution = OracleResolution {
                    market_id,
                    outcome,
                    confidence,
                    reasoning,
                    sources,
                    resolved_at,
                };

                // Record resolution
                if let Err(e) = self.state.record_resolution(resolution).await {
                    return OracleResponse::Error(e);
                }

                // Call market app to resolve the market
                let market_app_id = self.runtime.application_parameters().market_app;
                let call = market::MarketOperation::ResolveMarket { market_id, outcome };
                
                self.runtime.call_application(true, market_app_id, &call);

                OracleResponse::Ok
            }

            OracleOperation::ResolveMarketManual { market_id, outcome } => {
                let resolver = match self.runtime.authenticated_signer() {
                    Some(s) => s,
                    None => return OracleResponse::Error("Must be authenticated".to_string()),
                };

                // Check if resolver is registered oracle
                if !self.state.is_oracle(&resolver).await {
                    return OracleResponse::Error("Not a registered oracle".to_string());
                }

                let resolved_at = self.runtime.system_time();

                let resolution = OracleResolution {
                    market_id,
                    outcome,
                    confidence: 100.0,
                    reasoning: "Manual resolution".to_string(),
                    sources: vec![],
                    resolved_at,
                };

                if let Err(e) = self.state.record_resolution(resolution).await {
                    return OracleResponse::Error(e);
                }

                // Call market app to resolve the market
                let market_app_id = self.runtime.application_parameters().market_app;
                let call = market::MarketOperation::ResolveMarket { market_id, outcome };
                
                self.runtime.call_application(true, market_app_id, &call);

                OracleResponse::Ok
            }

            OracleOperation::RegisterOracle => {
                let oracle = match self.runtime.authenticated_signer() {
                    Some(s) => s,
                    None => return OracleResponse::Error("Must be authenticated".to_string()),
                };

                match self.state.register_oracle(oracle).await {
                    Ok(_) => OracleResponse::Ok,
                    Err(e) => OracleResponse::Error(e),
                }
            }
        }
    }

    async fn execute_message(&mut self, _message: Self::Message) {
        panic!("Oracle app does not handle messages");
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}
