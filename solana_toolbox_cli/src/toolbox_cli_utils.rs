use std::str::FromStr;

use solana_cli_config::Config;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::signature::Keypair;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_cli_error::ToolboxCliError;

pub struct ToolboxCliUtils {}

impl ToolboxCliUtils {
    pub fn new_endpoint(
        config: &Config,
    ) -> Result<ToolboxEndpoint, ToolboxCliError> {
        Ok(ToolboxEndpoint::new_rpc_with_url_and_commitment(
            &config.json_rpc_url,
            CommitmentConfig::from_str(&config.commitment)?,
        ))
    }

    pub fn load_keypair(path: &str) -> Result<Keypair, ToolboxCliError> {
        read_keypair_file(path).ok().ok_or_else(|| {
            ToolboxCliError::Custom(
                "Could not read default solana payer".to_string(),
            )
        })
    }
}
