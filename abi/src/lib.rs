use async_graphql::{Enum, SimpleObject};
use linera_sdk::{
    linera_base_types::{AccountOwner, Amount, ChainId, Timestamp},
    graphql::GraphQLMutationRoot,
};
use serde::{Deserialize, Serialize};

// =============================================================================
// MARKET TYPES
// =============================================================================

#[derive(Clone, Debug, Deserialize, Serialize, SimpleObject)]
pub struct Market {
    pub id: u64,
    pub creator: AccountOwner,
    pub question: String,
    pub description: String,
    pub category: MarketCategory,
    pub status: MarketStatus,
    pub created_at: Timestamp,
    pub closes_at: Timestamp,
    pub resolved_at: Option<Timestamp>,
    pub outcome: Option<Outcome>,
    pub total_pool: Amount,
    pub up_pool: Amount,
    pub down_pool: Amount,
    pub oracle_mode: OracleMode,
    pub resolution_source: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Enum, PartialEq, Eq, Copy)]
pub enum MarketCategory {
    Crypto,
    Sports,
    Politics,
    Weather,
    AI,
    Custom,
}

#[derive(Clone, Debug, Deserialize, Serialize, Enum, PartialEq, Eq, Copy)]
pub enum MarketStatus {
    Active,
    Closed,
    Resolved,
    Cancelled,
}

#[derive(Clone, Debug, Deserialize, Serialize, Enum, PartialEq, Eq, Copy)]
pub enum Outcome {
    Up,
    Down,
}

#[derive(Clone, Debug, Deserialize, Serialize, Enum, PartialEq, Eq, Copy)]
pub enum OracleMode {
    /// AI analyzes and resolves automatically
    AI,
    /// Manual resolution by creator
    Manual,
    /// Decentralized oracle on personal chain
    Decentralized,
}

impl MarketCategory {
    pub fn as_str(&self) -> &str {
        match self {
            MarketCategory::Crypto => "Crypto",
            MarketCategory::Sports => "Sports",
            MarketCategory::Politics => "Politics",
            MarketCategory::Weather => "Weather",
            MarketCategory::AI => "AI",
            MarketCategory::Custom => "Custom",
        }
    }
}

// =============================================================================
// BET TYPES
// =============================================================================

#[derive(Clone, Debug, Deserialize, Serialize, SimpleObject)]
pub struct Bet {
    pub market_id: u64,
    pub bettor: AccountOwner,
    pub prediction: Outcome,
    pub amount: Amount,
    pub placed_at: Timestamp,
    pub claimed: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, SimpleObject)]
pub struct Payout {
    pub market_id: u64,
    pub winner: AccountOwner,
    pub amount: Amount,
}

// =============================================================================
// AI ORACLE TYPES
// =============================================================================

#[derive(Clone, Debug, Deserialize, Serialize, SimpleObject)]
pub struct OracleResolution {
    pub market_id: u64,
    pub outcome: Outcome,
    pub confidence: f32,
    pub reasoning: String,
    pub sources: Vec<String>,
    pub resolved_at: Timestamp,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AIAnalysis {
    pub outcome: Outcome,
    pub confidence: f32,
    pub reasoning: String,
    pub data_sources: Vec<DataSource>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DataSource {
    pub name: String,
    pub url: String,
    pub data: String,
    pub timestamp: Timestamp,
}

// =============================================================================
// AI AGENT TYPES
// =============================================================================

#[derive(Clone, Debug, Deserialize, Serialize, SimpleObject)]
pub struct AIAgent {
    pub id: u64,
    pub name: String,
    pub strategy: AgentStrategy,
    pub total_bets: u64,
    pub wins: u64,
    pub losses: u64,
    pub total_profit: Amount,
    pub accuracy: f32,
    pub active: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, Enum, PartialEq, Eq, Copy)]
pub enum AgentStrategy {
    Conservative,
    Moderate,
    Aggressive,
    Contrarian,
    AIAnalysis,
}

// =============================================================================
// CONSTANTS
// =============================================================================

/// Minimum bet amount: 0.1 tokens (100,000 atto)
pub const MIN_BET_AMOUNT: u128 = 100_000;

/// Market creation fee: 1 token
pub const MARKET_CREATION_FEE: u128 = 1_000_000;

/// Platform fee: 5%
pub const PLATFORM_FEE_PERCENT: u8 = 5;

/// Min market duration: 1 minute
pub const MIN_MARKET_DURATION_MICROS: u64 = 60_000_000;

/// Max market duration: 7 days
pub const MAX_MARKET_DURATION_MICROS: u64 = 604_800_000_000;

/// Daily bonus amount: 100 tokens
pub const DAILY_BONUS_AMOUNT: u128 = 100_000_000;

/// Daily bonus cooldown: 24 hours
pub const DAILY_BONUS_COOLDOWN_MICROS: u64 = 86_400_000_000;

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

impl Market {
    pub fn is_active(&self, current_time: Timestamp) -> bool {
        self.status == MarketStatus::Active && current_time < self.closes_at
    }

    pub fn is_closed(&self, current_time: Timestamp) -> bool {
        self.status == MarketStatus::Active && current_time >= self.closes_at
    }

    pub fn can_bet(&self, current_time: Timestamp) -> bool {
        self.status == MarketStatus::Active && current_time < self.closes_at
    }

    pub fn odds_up(&self) -> f64 {
        if self.total_pool.is_zero() {
            return 0.5;
        }
        let total = u128::from(self.total_pool) as f64;
        let up = u128::from(self.up_pool) as f64;
        up / total
    }

    pub fn odds_down(&self) -> f64 {
        1.0 - self.odds_up()
    }
}

impl AIAgent {
    pub fn win_rate(&self) -> f32 {
        if self.total_bets == 0 {
            return 0.0;
        }
        (self.wins as f32 / self.total_bets as f32) * 100.0
    }

    pub fn update_stats(&mut self, won: bool, profit: Amount) {
        self.total_bets += 1;
        if won {
            self.wins += 1;
        } else {
            self.losses += 1;
        }
        self.total_profit = self.total_profit.saturating_add(profit);
        self.accuracy = self.win_rate();
    }
}
