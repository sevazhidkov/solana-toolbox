use clap::Parser;
use clap::Subcommand;
use solana_sdk::signature::Keypair;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_cli_account::ToolboxCliAccountArgs;
use crate::toolbox_cli_error::ToolboxCliError;
use crate::toolbox_cli_idl::ToolboxCliIdlArgs;

#[derive(Debug, Clone, Parser)]
pub struct ToolboxCliArgs {
    #[command(subcommand)]
    command: ToolboxCliCommand,
}

impl ToolboxCliArgs {
    pub async fn process(&self) -> Result<(), ToolboxCliError> {
        let mut endpoint = ToolboxEndpoint::new_devnet().await; // TODO - proper endpoint
        let payer = Keypair::new(); // TODO - get local wallet
        self.command.process(&mut endpoint, &payer).await
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum ToolboxCliCommand {
    Account(ToolboxCliAccountArgs),
    Idl(ToolboxCliIdlArgs),
}

impl ToolboxCliCommand {
    pub async fn process(
        &self,
        endpoint: &mut ToolboxEndpoint,
        payer: &Keypair,
    ) -> Result<(), ToolboxCliError> {
        match self {
            ToolboxCliCommand::Account(args) => {
                args.process(endpoint, payer).await
            },
            ToolboxCliCommand::Idl(args) => args.process(endpoint, payer).await,
        }
    }
}
