#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::sync::Arc;

use async_graphql::{EmptySubscription, Object, Request, Response, Schema};
use linera_sdk::{
    abi::WithServiceAbi,
    views::View,
    Service, ServiceRuntime,
};
use self::state::OracleState;

pub struct OracleService {
    state: Arc<OracleState>,
}

linera_sdk::service!(OracleService);

impl WithServiceAbi for OracleService {
    type Abi = oracle::OracleAbi;
}

impl Service for OracleService {
    type Parameters = oracle::OracleParameters;

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = OracleState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        OracleService {
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
    /// Placeholder mutation
    async fn placeholder(&self) -> bool {
        true
    }
}
