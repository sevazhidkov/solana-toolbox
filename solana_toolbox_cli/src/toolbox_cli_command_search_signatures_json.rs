use std::str::FromStr;

use clap::Args;
use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signature};
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Args)]
pub struct ToolboxCliCommandSearchSignaturesJsonArgs {
    with_address: String,
    start_before_signature: Option<String>,
    rewind_until_signature: Option<String>,
    limit: Option<usize>,
}

impl ToolboxCliCommandSearchSignaturesJsonArgs {
    pub async fn process(
        &self,
        endpoint: &mut ToolboxEndpoint,
        _payer: &Keypair,
    ) -> Result<(), ToolboxCliError> {
        let with_address = Pubkey::from_str(&self.with_address).unwrap();
        let start_before = self
            .start_before_signature
            .as_ref()
            .map(|signature| Signature::from_str(signature))
            .transpose()?;
        let rewind_until = self
            .rewind_until_signature
            .as_ref()
            .map(|signature| Signature::from_str(signature))
            .transpose()?;
        let signatures = endpoint
            .search_signatures(
                &with_address,
                start_before,
                rewind_until,
                self.limit.unwrap_or(10),
            )
            .await?;
        let json = json!(signatures
            .iter()
            .map(|signature| signature.to_string())
            .collect::<Vec<_>>());
        println!("{}", serde_json::to_string(&json)?);
        Ok(())
    }
}
