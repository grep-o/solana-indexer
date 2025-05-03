use axum::response::IntoResponse;
use indexer::cache::TransactionsCache;

use super::dynamic_response::DynamicResponse;

pub struct StatsController;

impl StatsController {
    pub async fn get() -> impl IntoResponse {
        let meta = TransactionsCache::get_meta();
        DynamicResponse::success_data(meta)
    }
}
