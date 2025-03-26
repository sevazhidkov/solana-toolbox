use std::str::FromStr;

use clap::Args;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_cli_config::ToolboxCliConfig;
use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Args)]
#[command(about = "Resolve a program's IDL")]
pub struct ToolboxCliCommandProgramArgs {
    #[arg(value_name = "PROGRAM_ID", help = "The Program ID pubkey in base58")]
    program_id: String,
    // TODO (FAR) - support selecting the output format version
}

impl ToolboxCliCommandProgramArgs {
    pub async fn process(
        &self,
        config: &ToolboxCliConfig,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = config.create_endpoint().await?;
        let mut idl_service = config.create_resolver().await?;
        let program_id = Pubkey::from_str(&self.program_id).unwrap();
        let idl_program = idl_service
            .resolve_program(&mut endpoint, &program_id)
            .await?
            .unwrap_or_default();
        println!("{}", serde_json::to_string(&idl_program.export(false))?);
        Ok(())
    }
}
