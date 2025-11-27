use async_graphql::SimpleObject;
use linera_sdk::views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext};
use oracle_ai_abi::AIAgent;

#[derive(RootView, SimpleObject)]
#[view(context = ViewStorageContext)]
pub struct AIAgentState {
    /// All AI agents
    pub agents: MapView<u64, AIAgent>,

    /// Next agent ID
    pub next_agent_id: RegisterView<u64>,
}

impl AIAgentState {
    /// Create a new agent
    pub async fn create_agent(&mut self, agent: AIAgent) -> Result<u64, String> {
        let id = *self.next_agent_id.get();

        let mut agent_with_id = agent;
        agent_with_id.id = id;

        self.agents
            .insert(&id, agent_with_id)
            .map_err(|e| format!("Failed to create agent: {}", e))?;

        self.next_agent_id.set(id + 1);
        Ok(id)
    }

    /// Get an agent
    pub async fn get_agent(&self, id: u64) -> Result<AIAgent, String> {
        self.agents
            .get(&id)
            .await
            .map_err(|e| format!("Failed to get agent: {}", e))?
            .ok_or_else(|| format!("Agent {} not found", id))
    }

    /// Update an agent
    pub async fn update_agent(&mut self, agent: AIAgent) -> Result<(), String> {
        self.agents
            .insert(&agent.id.clone(), agent)
            .map_err(|e| format!("Failed to update agent: {}", e))
    }
}
