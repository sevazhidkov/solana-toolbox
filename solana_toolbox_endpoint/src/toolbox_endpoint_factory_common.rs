use solana_sdk::commitment_config::CommitmentConfig;

use crate::toolbox_endpoint::ToolboxEndpoint;

impl ToolboxEndpoint {
    pub async fn new_memnet() -> ToolboxEndpoint {
        ToolboxEndpoint::new_program_test().await
    }

    pub async fn new_devnet() -> ToolboxEndpoint {
        ToolboxEndpoint::new_rpc_with_url_or_moniker_and_commitment(
            "devnet",
            CommitmentConfig::confirmed(),
        )
    }

    pub async fn new_mainnet() -> ToolboxEndpoint {
        ToolboxEndpoint::new_rpc_with_url_or_moniker_and_commitment(
            "mainnet-beta",
            CommitmentConfig::confirmed(),
        )
    }
}
