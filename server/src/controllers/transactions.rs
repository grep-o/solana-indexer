use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
};
use indexer::cache::TransactionsCache;

use super::{dynamic_response::DynamicResponse, types::RecentQuery};

pub struct TransactionsController;

impl TransactionsController {
    pub async fn get_transaction_by_hash(Path(tx_hash): Path<String>) -> impl IntoResponse {
        match TransactionsCache::get_by_hash(&tx_hash) {
            Some(tx) => DynamicResponse::success_data(tx),
            None => DynamicResponse::failure_message("Transaction not found", Some(StatusCode::NOT_FOUND)),
        }
    }

    pub async fn get_recent_transactions(Query(recent_query): Query<RecentQuery>) -> impl IntoResponse {
        DynamicResponse::success_data(TransactionsCache::get_recent(recent_query.limit))
    }
}
