use clap::Args;
use serde_json::Value;

use crate::toolbox_cli_context::ToolboxCliContext;
use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Args)]
#[command(about = "Resolve a program's IDL")]
pub struct ToolboxCliCommandProgramArgs {
    #[arg(value_name = "PROGRAM_ID", help = "The Program ID pubkey in base58")]
    program_id: String,
    #[arg(long, action)]
    backward_compatibility: bool,
}

impl ToolboxCliCommandProgramArgs {
    pub async fn process(
        &self,
        context: &ToolboxCliContext,
    ) -> Result<Value, ToolboxCliError> {
        let mut endpoint = context.create_endpoint().await?;
        let mut idl_service = context.create_service().await?;
        let program_id = context.parse_key(&self.program_id)?.address();
        let idl_program = idl_service
            .resolve_program(&mut endpoint, &program_id)
            .await?
            .unwrap_or_default();
        Ok(idl_program.export(self.backward_compatibility))
    }
}
