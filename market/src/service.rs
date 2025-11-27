#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::sync::Arc;

use async_graphql::{EmptySubscription, Object, Request, Response, Schema};
use linera_sdk::{
    abi::WithServiceAbi,
    views::View,
    Service, ServiceRuntime,
};
use self::state::MarketState;

pub struct MarketService {
    state: Arc<MarketState>,
}

linera_sdk::service!(MarketService);

impl WithServiceAbi for MarketService {
    type Abi = market::MarketAbi;
}

impl Service for MarketService {
    type Parameters = market::MarketParameters;

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = MarketState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        MarketService {
            state: Arc::new(state),
        }
    }

    async fn handle_query(&self, request: Request) -> Response {
        let schema = Schema::build(
            self.state.clone(),
            MutationRoot,
            EmptySubscription,
        )
        .finish();

        schema.execute(request).await
    }
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_market(
        &self,
        question: String,
        description: String,
        category: oracle_ai_abi::MarketCategory,
        duration_minutes: u64,
        oracle_mode: oracle_ai_abi::OracleMode,
    ) -> Vec<u8> {
        let op = market::MarketOperation::CreateMarket {
            question,
            description,
            category,
            duration_minutes,
            oracle_mode,
        };
        bcs::to_bytes(&op).unwrap()
    }

    async fn place_bet(
        &self,
        market_id: u64,
        prediction: oracle_ai_abi::Outcome,
        amount: linera_sdk::linera_base_types::Amount,
    ) -> Vec<u8> {
        let op = market::MarketOperation::PlaceBet {
            market_id,
            prediction,
            amount,
        };
        bcs::to_bytes(&op).unwrap()
    }
}
