use std::str::FromStr;

use clap::Args;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdlResolver;

use crate::toolbox_cli_config::ToolboxCliConfig;
use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Args)]
pub struct ToolboxCliCommandIdlProgramArgs {
    program_id: String,
}

impl ToolboxCliCommandIdlProgramArgs {
    pub async fn process(
        &self,
        config: &ToolboxCliConfig,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = config.create_endpoint()?;
        let program_id = Pubkey::from_str(&self.program_id).unwrap();
        let idl_program = ToolboxIdlResolver::new()
            .resolve_program(&mut endpoint, &program_id)
            .await?;
        // TODO - add a new description JSON format output
        println!("{}", serde_json::to_string(&idl_program.export(false))?);
        Ok(())
    }
}
