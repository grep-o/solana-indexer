use std::sync::LazyLock;

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

pub static RPC_CLIENT: LazyLock<RpcClient> = LazyLock::new(|| {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed())
});
