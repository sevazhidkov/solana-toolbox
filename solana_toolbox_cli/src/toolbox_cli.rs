use clap::Parser;
use clap::Subcommand;
use solana_sdk::signature::Keypair;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_cli_command_get_account_json::ToolboxCliCommandGetAccountJsonArgs;
use crate::toolbox_cli_command_get_execution_json::ToolboxCliCommandGetExecutionJsonArgs;
use crate::toolbox_cli_command_idl_decompiled_account_json::ToolboxCliCommandIdlDecompiledAccountJsonArgs;
use crate::toolbox_cli_command_idl_decompiled_execution_json::ToolboxCliCommandIdlDecompiledExecutionJsonArgs;
use crate::toolbox_cli_command_inspect_account::ToolboxCliCommandInspectAccountArgs;
use crate::toolbox_cli_command_search_addresses_json::ToolboxCliCommandSearchAddressesJsonArgs;
use crate::toolbox_cli_command_search_signatures_json::ToolboxCliCommandSearchSignaturesJsonArgs;
use crate::toolbox_cli_error::ToolboxCliError;

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
    GetAccountJson(ToolboxCliCommandGetAccountJsonArgs),
    GetExecutionJson(ToolboxCliCommandGetExecutionJsonArgs),
    IdlDecompiledAccountJson(ToolboxCliCommandIdlDecompiledAccountJsonArgs),
    IdlDecompiledExecutionJson(ToolboxCliCommandIdlDecompiledExecutionJsonArgs),
    InspectAccount(ToolboxCliCommandInspectAccountArgs),
    SearchAddressesJson(ToolboxCliCommandSearchAddressesJsonArgs),
    SearchSignaturesJson(ToolboxCliCommandSearchSignaturesJsonArgs),
}

impl ToolboxCliCommand {
    pub async fn process(
        &self,
        endpoint: &mut ToolboxEndpoint,
        payer: &Keypair,
    ) -> Result<(), ToolboxCliError> {
        match self {
            ToolboxCliCommand::GetAccountJson(args) => {
                args.process(endpoint, payer).await
            },
            ToolboxCliCommand::GetExecutionJson(args) => {
                args.process(endpoint, payer).await
            },
            ToolboxCliCommand::IdlDecompiledAccountJson(args) => {
                args.process(endpoint, payer).await
            },
            ToolboxCliCommand::IdlDecompiledExecutionJson(args) => {
                args.process(endpoint, payer).await
            },
            ToolboxCliCommand::InspectAccount(args) => {
                args.process(endpoint, payer).await
            },
            ToolboxCliCommand::SearchAddressesJson(args) => {
                args.process(endpoint, payer).await
            },
            ToolboxCliCommand::SearchSignaturesJson(args) => {
                args.process(endpoint, payer).await
            },
        }
    }
}
