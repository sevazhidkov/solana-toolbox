use solana_sdk::commitment_config::CommitmentConfig;

use crate::toolbox_endpoint::ToolboxEndpoint;

impl ToolboxEndpoint {
    pub async fn new_memnet() -> ToolboxEndpoint {
        ToolboxEndpoint::new_program_test().await
    }

    pub async fn new_devnet() -> ToolboxEndpoint {
        ToolboxEndpoint::new_rpc_with_url_and_commitment(
            "https://api.devnet.solana.com",
            CommitmentConfig::confirmed(),
        )
    }

    pub async fn new_mainnet() -> ToolboxEndpoint {
        ToolboxEndpoint::new_rpc_with_url_and_commitment(
            "https://api.mainnet-beta.solana.com",
            CommitmentConfig::confirmed(),
        )
    }
}
