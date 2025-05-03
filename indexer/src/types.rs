use serde::Serialize;
use solana_transaction_status::UiParsedMessage;

pub type TransactionHash = String;

#[derive(Clone, Serialize)]
pub struct Transaction {
    pub hash: String,
    pub messages: UiParsedMessage,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct Meta {
    pub previous_blockhash: String,
    pub blockhash: String,
    pub slot: u64,
    pub parent_slot: u64,
    pub block_time: Option<i64>,
    pub block_height: Option<u64>,
    pub transaction_count: usize,
}
