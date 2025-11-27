use async_graphql::SimpleObject;
use linera_sdk::{
    linera_base_types::{AccountOwner, Amount},
    views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext},
};
use serde::{Deserialize, Serialize};

use token::DailyBonus;

/// Token application state
#[derive(RootView, SimpleObject)]
#[view(context = ViewStorageContext)]
pub struct TokenState {
    /// Account balances
    pub accounts: MapView<AccountOwner, Amount>,

    /// Daily bonus tracking per user
    pub daily_bonuses: MapView<AccountOwner, DailyBonus>,

    /// Total supply
    pub total_supply: RegisterView<Amount>,
}

impl TokenState {
    /// Get balance for an account
    pub async fn balance(&self, owner: &AccountOwner) -> Amount {
        self.accounts
            .get(owner)
            .await
            .unwrap_or(None)
            .unwrap_or(Amount::ZERO)
    }

    /// Credit an account
    pub async fn credit(&mut self, owner: &AccountOwner, amount: Amount) -> Result<(), String> {
        let current = self.balance(owner).await;
        let new_balance = current.saturating_add(amount);
        self.accounts.insert(owner, new_balance).map_err(|e| e.to_string())?;

        let total = self.total_supply.get();
        self.total_supply.set(total.saturating_add(amount));

        Ok(())
    }

    /// Debit an account
    pub async fn debit(&mut self, owner: &AccountOwner, amount: Amount) -> Result<(), String> {
        let current = self.balance(owner).await;

        if current < amount {
            return Err(format!(
                "Insufficient balance: {} < {}",
                current, amount
            ));
        }

        let new_balance = current.saturating_sub(amount);
        self.accounts.insert(owner, new_balance).map_err(|e| e.to_string())?;

        let total = self.total_supply.get();
        self.total_supply.set(total.saturating_sub(amount));

        Ok(())
    }

    /// Transfer between accounts
    pub async fn transfer(
        &mut self,
        from: &AccountOwner,
        to: &AccountOwner,
        amount: Amount,
    ) -> Result<(), String> {
        self.debit(from, amount).await?;
        self.credit(to, amount).await?;
        Ok(())
    }
}
