use std::str::FromStr;

use clap::Args;
use serde_json::json;
use solana_cli_config::Config;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_cli_error::ToolboxCliError;
use crate::toolbox_cli_utils::ToolboxCliUtils;

#[derive(Debug, Clone, Args)]
pub struct ToolboxCliCommandSearchAddressesArgs {
    program_address: String,
    data_len: Option<usize>,
    // data_chunks: Vec<String>,
    // TODO - this should support memcpm
}

impl ToolboxCliCommandSearchAddressesArgs {
    pub async fn process(
        &self,
        config: &Config,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = ToolboxCliUtils::new_endpoint(config)?;
        let program_address = Pubkey::from_str(&self.program_address).unwrap();
        let addresses = endpoint
            .search_addresses(&program_address, self.data_len, &[])
            .await?;
        let json = json!(addresses
            .iter()
            .map(|address| address.to_string())
            .collect::<Vec<_>>());
        println!("{}", serde_json::to_string(&json)?);
        Ok(())
    }
}
