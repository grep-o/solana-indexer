use std::str::FromStr;

use axum::{extract::Path, response::IntoResponse};
use indexer::client;
use solana_account_decoder::UiAccountData;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_sdk::pubkey::Pubkey;

use super::{
    dynamic_response::DynamicResponse,
    types::{AccountResponse, Token},
};

pub struct AccountController;

impl AccountController {
    pub async fn get(Path(address): Path<String>) -> impl IntoResponse {
        let pubkey = match Pubkey::from_str(&address) {
            Ok(pubkey) => pubkey,
            Err(err) => return DynamicResponse::failure_message(&err.to_string(), None),
        };

        match client::RPC_CLIENT.get_account(&pubkey).await {
            Ok(account) => {
                let balance = match client::RPC_CLIENT.get_balance(&pubkey).await {
                    Ok(balance) => balance,
                    Err(err) => return DynamicResponse::failure_message(&err.to_string(), None),
                };

                let token_accounts = match client::RPC_CLIENT
                    .get_token_accounts_by_owner(&pubkey, TokenAccountsFilter::ProgramId(spl_token::id()))
                    .await
                {
                    Ok(token_accounts) => token_accounts,
                    Err(err) => return DynamicResponse::failure_message(&err.to_string(), None),
                };

                let mut tokens = Vec::new();

                for token_account in token_accounts {
                    if let UiAccountData::Json(token) = token_account.account.data {
                        if let Some(info) = token.parsed.get("info") {
                            if let Some(token_amount) = info.get("tokenAmount") {
                                tokens.push(Token {
                                    mint: info.get("mint").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
                                    decimals: token_amount.get("decimals").and_then(|v| v.as_u64()).map(|v| v as u8).unwrap_or(0),
                                    balance: token_amount.get("amount").and_then(|v| v.as_str()).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0),
                                    balance_ui: token_amount.get("uiAmount").and_then(|v| v.as_f64()).unwrap_or(0.0),
                                })
                            }
                        };
                    }
                }

                DynamicResponse::success_data(AccountResponse {
                    account,
                    balance,
                    balance_ui: balance as f64 / 1e9,
                    tokens,
                })
            }
            Err(err) => DynamicResponse::failure_message(&err.to_string(), None),
        }
    }
}
