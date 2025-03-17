use std::str::FromStr;

use clap::Args;
use solana_cli_config::Config;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdlResolver;

use crate::toolbox_cli_error::ToolboxCliError;
use crate::toolbox_cli_utils::ToolboxCliUtils;

#[derive(Debug, Clone, Args)]
pub struct ToolboxCliCommandIdlResolveProgramArgs {
    program_id: String,
}

impl ToolboxCliCommandIdlResolveProgramArgs {
    pub async fn process(
        &self,
        config: &Config,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = ToolboxCliUtils::new_endpoint(config)?;
        let program_id = Pubkey::from_str(&self.program_id).unwrap();
        let idl_program = ToolboxIdlResolver::new()
            .resolve_idl_program(&mut endpoint, &program_id)
            .await?;
        // TODO - add a new description JSON format output
        println!("{}", serde_json::to_string(&idl_program.as_json(false))?);
        Ok(())
    }
}
