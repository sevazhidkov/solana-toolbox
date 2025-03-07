use clap::Args;
use clap::Subcommand;
use solana_sdk::signature::Keypair;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Args)]
pub struct ToolboxCliIdlArgs {
    #[command(subcommand)]
    command: ToolboxCliIdlCommand,
}

impl ToolboxCliIdlArgs {
    pub async fn process(
        &self,
        endpoint: &mut ToolboxEndpoint,
        payer: &Keypair,
    ) -> Result<(), ToolboxCliError> {
        self.command.process(endpoint, payer).await
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum ToolboxCliIdlCommand {
    Data {
        address: String,
    },
    Json {
        address: String,
        path: Option<String>,
    },
}

impl ToolboxCliIdlCommand {
    pub async fn process(
        &self,
        endpoint: &mut ToolboxEndpoint,
        payer: &Keypair,
    ) -> Result<(), ToolboxCliError> {
        match self {
            ToolboxCliIdlCommand::Data { address } => todo!(),
            ToolboxCliIdlCommand::Json { address, path } => todo!(),
        }
    }
}
