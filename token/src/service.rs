#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::sync::Arc;

use async_graphql::{EmptySubscription, Object, Request, Response, Schema};
use linera_sdk::{
    abi::WithServiceAbi,
    views::View,
    Service, ServiceRuntime,
};
use self::state::TokenState;

pub struct TokenService {
    state: Arc<TokenState>,
}

linera_sdk::service!(TokenService);

impl WithServiceAbi for TokenService {
    type Abi = token::TokenAbi;
}

impl Service for TokenService {
    type Parameters = token::TokenParameters;

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = TokenState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        TokenService {
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
    /// Get the current block height (placeholder for future features)
    async fn placeholder(&self) -> bool {
        true
    }
}
