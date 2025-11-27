#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::sync::Arc;

use async_graphql::{EmptySubscription, Object, Request, Response, Schema};
use linera_sdk::{
    abi::WithServiceAbi,
    views::View,
    Service, ServiceRuntime,
};
use self::state::AIAgentState;

pub struct AIAgentService {
    state: Arc<AIAgentState>,
}

linera_sdk::service!(AIAgentService);

impl WithServiceAbi for AIAgentService {
    type Abi = ai_agent::AIAgentAbi;
}

impl Service for AIAgentService {
    type Parameters = ai_agent::AIAgentParameters;

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = AIAgentState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        AIAgentService {
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
    async fn placeholder(&self) -> bool {
        true
    }
}
