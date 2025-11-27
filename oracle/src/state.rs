use async_graphql::SimpleObject;
use linera_sdk::{
    linera_base_types::{AccountOwner, Amount},
    views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext},
};
use oracle_ai_abi::OracleResolution;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, SimpleObject)]
pub struct PriceFeed {
    pub symbol: String,
    pub price: Amount,
    pub timestamp: u64,
}

#[derive(RootView, SimpleObject)]
#[view(context = ViewStorageContext)]
pub struct OracleState {
    /// Price feeds
    pub price_feeds: MapView<String, PriceFeed>,

    /// Oracle resolutions
    pub resolutions: MapView<u64, OracleResolution>,

    /// Registered oracles
    pub oracles: MapView<AccountOwner, bool>,

    /// Resolution count
    pub resolution_count: RegisterView<u64>,
}

impl OracleState {
    /// Submit a price
    pub async fn submit_price(&mut self, symbol: String, price: Amount, timestamp: u64) -> Result<(), String> {
        let feed = PriceFeed {
            symbol: symbol.clone(),
            price,
            timestamp,
        };

        self.price_feeds
            .insert(&symbol, feed)
            .map_err(|e| format!("Failed to submit price: {}", e))
    }

    /// Get latest price
    pub async fn get_price(&self, symbol: &str) -> Option<Amount> {
        self.price_feeds
            .get(symbol)
            .await
            .ok()?
            .map(|feed| feed.price)
    }

    /// Record a resolution
    pub async fn record_resolution(&mut self, resolution: OracleResolution) -> Result<(), String> {
        self.resolutions
            .insert(&resolution.market_id.clone(), resolution)
            .map_err(|e| format!("Failed to record resolution: {}", e))?;

        let count = *self.resolution_count.get();
        self.resolution_count.set(count + 1);

        Ok(())
    }

    /// Register as oracle
    pub async fn register_oracle(&mut self, oracle: AccountOwner) -> Result<(), String> {
        self.oracles
            .insert(&oracle, true)
            .map_err(|e| format!("Failed to register oracle: {}", e))
    }

    /// Check if account is registered oracle
    pub async fn is_oracle(&self, account: &AccountOwner) -> bool {
        self.oracles
            .get(account)
            .await
            .unwrap_or(None)
            .unwrap_or(false)
    }
}
