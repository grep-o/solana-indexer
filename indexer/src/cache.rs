use tracing::info;

use crate::types::{Meta, Transaction, TransactionHash};
use std::collections::{HashMap, VecDeque};
use std::sync::{LazyLock, RwLock};

#[derive(Default)]
pub struct TransactionsCache {
    pub meta: Meta,
    pub raw: VecDeque<TransactionHash>,
    pub map: HashMap<TransactionHash, Transaction>,
}

static INSTANCE: LazyLock<RwLock<TransactionsCache>> = LazyLock::new(|| RwLock::new(TransactionsCache::default()));

impl TransactionsCache {
    const MAX_RAW_LEN: usize = 1000;

    pub fn get_by_hash(tx_hash: &TransactionHash) -> Option<Transaction> {
        let cache = INSTANCE.read().unwrap();
        cache.map.get(tx_hash).cloned()
    }

    pub fn get_recent(limit: usize) -> Vec<Transaction> {
        let cache = INSTANCE.read().unwrap();

        cache.raw.iter().take(limit).fold(Vec::with_capacity(limit), |mut acc, hash| {
            if let Some(tx) = cache.map.get(hash) {
                acc.push(tx.clone());
            }

            acc
        })
    }

    pub fn insert(tx_hash: TransactionHash, transaction: Transaction) {
        let mut cache = INSTANCE.write().unwrap();
        cache.raw.push_front(tx_hash.clone());
        if cache.raw.len() > Self::MAX_RAW_LEN {
            if let Some(last_hash) = cache.raw.pop_back() {
                cache.map.remove(&last_hash);
            }
        }

        cache.map.insert(tx_hash, transaction);
    }

    pub fn update_meta(meta: Meta) {
        let mut cache = INSTANCE.write().unwrap();
        info!("Meta Updated: {:?}", meta);
        cache.meta = meta;
    }

    pub fn get_meta() -> Meta {
        let cache = INSTANCE.read().unwrap();
        cache.meta.clone()
    }
}
