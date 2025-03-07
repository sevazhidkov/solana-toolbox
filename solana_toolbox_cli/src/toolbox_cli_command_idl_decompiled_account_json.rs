use std::str::FromStr;

use clap::Args;
use serde_json::json;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_idl::ToolboxIdl;

use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Args)]
pub struct ToolboxCliCommandIdlDecompiledAccountJsonArgs {
    address: String,
    // TODO - should support loading a custom IDL ?
}

impl ToolboxCliCommandIdlDecompiledAccountJsonArgs {
    pub async fn process(
        &self,
        endpoint: &mut ToolboxEndpoint,
        _payer: &Keypair,
    ) -> Result<(), ToolboxCliError> {
        let address = Pubkey::from_str(&self.address).unwrap();
        let account = endpoint.get_account(&address).await?.unwrap(); // TODO - unwrap util
        let idl = ToolboxIdl::get_for_program_id(endpoint, &account.owner)
            .await?
            .unwrap(); // TODO - handle unwrap
        let decompiled = idl.decompile_account(&account.data).unwrap();
        let json = &json!({
            "name": decompiled.name,
            "state": decompiled.state,
        });
        println!("{}", serde_json::to_string(&json)?);
        Ok(())
    }
}
