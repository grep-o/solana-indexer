use indexer::runner;
use tracing_subscriber::EnvFilter;

pub mod controllers {
    pub mod account;
    pub mod dynamic_response;
    pub mod stats;
    pub mod transactions;
    pub mod types;
}
mod router;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let filter = EnvFilter::new("debug,h2=error,h2::codec=error,hyper=warn,reqwest=warn");
    tracing_subscriber::fmt().with_env_filter(filter).init();

    tokio::spawn(runner::watch_subscriptions());

    let service = router::main();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000".to_string()).await.unwrap();
    axum::serve(listener, service).await.unwrap();
}
