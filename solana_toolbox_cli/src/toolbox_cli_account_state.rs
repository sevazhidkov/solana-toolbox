use std::str::FromStr;

use clap::Args;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_idl::ToolboxIdl;

use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Args)]
pub struct ToolboxCliAccountStateArgs {
    address: String,
}

impl ToolboxCliAccountStateArgs {
    pub async fn process(
        &self,
        endpoint: &mut ToolboxEndpoint,
        _payer: &Keypair,
    ) -> Result<(), ToolboxCliError> {
        let key = Pubkey::from_str(&self.address).unwrap();
        let account = endpoint.get_account(&key).await?.unwrap(); // TODO - unwrap util
        let idl = ToolboxIdl::get_for_program_id(endpoint, &account.owner)
            .await?
            .unwrap();
        let decompiled = idl.decompile_account(&account.data).unwrap();
        println!("{}", serde_json::to_string_pretty(&decompiled.state)?);
        Ok(())
    }
}
