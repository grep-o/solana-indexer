use futures_util::stream::StreamExt;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter},
};
use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};
use solana_transaction_status::{EncodedTransaction, TransactionDetails, UiMessage, UiTransactionEncoding};
use tracing::{error, info};

use crate::{
    cache::TransactionsCache,
    types::{Meta, Transaction},
};

pub async fn watch_subscriptions() {
    // Spawn block subscription
    tokio::spawn({
        let websocket_url = std::env::var("WSS_URL").expect("Missing WSS_URL, set on .env");
        let client = PubsubClient::new(&websocket_url).await.expect("Failed to launch PubSub client");

        info!("WSS Client Ready...");

        async move {
            info!("WSS Client Subscribing...");
            match client
                .block_subscribe(
                    RpcBlockSubscribeFilter::All,
                    Some(RpcBlockSubscribeConfig {
                        commitment: Some(CommitmentConfig {
                            commitment: CommitmentLevel::Confirmed,
                        }),
                        transaction_details: Some(TransactionDetails::Full),
                        show_rewards: Some(false),
                        max_supported_transaction_version: Some(0),
                        encoding: Some(UiTransactionEncoding::JsonParsed),
                    }),
                )
                .await
            {
                Ok((mut notifications, _unsubscribe)) => {
                    info!("WSS Client Subscribed...");
                    while let Some(block) = notifications.next().await {
                        if let Some(err) = block.value.err {
                            error!("block_subscribe {err}")
                        }

                        if let Some(value) = block.value.block {
                            let transactions = value.transactions.unwrap_or_default();

                            TransactionsCache::update_meta(Meta {
                                previous_blockhash: value.previous_blockhash,
                                blockhash: value.blockhash,
                                slot: block.context.slot,
                                parent_slot: value.parent_slot,
                                block_time: value.block_time,
                                block_height: value.block_height,
                                transaction_count: transactions.len(),
                            });

                            for tx in transactions {
                                if let EncodedTransaction::Json(body) = tx.transaction {
                                    if let Some(hash) = body.signatures.first().map(|f| f.to_string()) {
                                        if let UiMessage::Parsed(messages) = body.message {
                                            TransactionsCache::insert(hash.to_string(), Transaction { hash, messages });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(err) => error!("Failed to subscribe: {err:?}"),
            }
        }
    });
}
