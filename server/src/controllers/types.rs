use serde::{Deserialize, Serialize};
use solana_sdk::account::Account;

#[derive(Deserialize)]
pub struct RecentQuery {
    pub limit: usize,
}

#[derive(Serialize)]
pub struct Token {
    pub mint: String,
    pub decimals: u8,
    pub balance: u64,
    pub balance_ui: f64,
}

#[derive(Serialize)]
pub struct AccountResponse {
    pub account: Account,
    pub balance: u64,
    pub balance_ui: f64,
    pub tokens: Vec<Token>,
}
