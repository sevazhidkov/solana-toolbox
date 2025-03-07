use clap::Args;
use clap::Subcommand;
use solana_sdk::signature::Keypair;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_cli_account_inspect::ToolboxCliAccountInspectArgs;
use crate::toolbox_cli_account_state::ToolboxCliAccountStateArgs;
use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Args)]
pub struct ToolboxCliAccountArgs {
    #[command(subcommand)]
    command: ToolboxCliAccountCommand,
}

impl ToolboxCliAccountArgs {
    pub async fn process(
        &self,
        endpoint: &mut ToolboxEndpoint,
        payer: &Keypair,
    ) -> Result<(), ToolboxCliError> {
        self.command.process(endpoint, payer).await
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum ToolboxCliAccountCommand {
    Inspect(ToolboxCliAccountInspectArgs),
    State(ToolboxCliAccountStateArgs),
}

impl ToolboxCliAccountCommand {
    pub async fn process(
        &self,
        endpoint: &mut ToolboxEndpoint,
        payer: &Keypair,
    ) -> Result<(), ToolboxCliError> {
        match self {
            ToolboxCliAccountCommand::Inspect(args) => {
                args.process(endpoint, payer).await
            },
            ToolboxCliAccountCommand::State(args) => {
                args.process(endpoint, payer).await
            },
        }
    }
}
