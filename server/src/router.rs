use axum::{Router, routing::get};

use crate::controllers::{account::AccountController, stats::StatsController, transactions::TransactionsController};

pub fn main() -> Router {
    Router::new()
        .route("/", get(StatsController::get))
        .route("/account/{address}", get(AccountController::get))
        .route("/signature/{signature}", get(TransactionsController::get_transaction_by_hash))
        .route("/recent", get(TransactionsController::get_recent_transactions))
}
