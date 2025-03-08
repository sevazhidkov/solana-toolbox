use clap::Parser;
use clap::Subcommand;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_cli_command_get_account::ToolboxCliCommandGetAccountArgs;
use crate::toolbox_cli_command_get_execution::ToolboxCliCommandGetExecutionArgs;
use crate::toolbox_cli_command_idl_decompile_account::ToolboxCliCommandIdlDecompileAccountArgs;
use crate::toolbox_cli_command_idl_decompile_execution::ToolboxCliCommandIdlDecompileExecutionArgs;
use crate::toolbox_cli_command_idl_describe::ToolboxCliCommandIdlDescribeArgs;
use crate::toolbox_cli_command_idl_process_instruction::ToolboxCliCommandIdlProcessInstructionArgs;
use crate::toolbox_cli_command_inspect_account::ToolboxCliCommandInspectAccountArgs;
use crate::toolbox_cli_command_search_addresses::ToolboxCliCommandSearchAddressesArgs;
use crate::toolbox_cli_command_search_signatures::ToolboxCliCommandSearchSignaturesArgs;
use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Parser)]
pub struct ToolboxCliArgs {
    #[arg(short, long)]
    cluster: Option<String>,
    #[command(subcommand)]
    command: ToolboxCliCommand,
}

impl ToolboxCliArgs {
    pub async fn process(&self) -> Result<(), ToolboxCliError> {
        // TODO - proper endpoint selection
        let mut endpoint = match &self.cluster {
            None => ToolboxEndpoint::new_devnet().await,
            Some(cluster) => ToolboxEndpoint::new_rpc_with_url_and_commitment(
                &cluster,
                CommitmentConfig::confirmed(),
            ),
        };
        self.command.process(&mut endpoint).await
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum ToolboxCliCommand {
    GetAccount(ToolboxCliCommandGetAccountArgs),
    GetExecution(ToolboxCliCommandGetExecutionArgs),
    IdlDecompileAccount(ToolboxCliCommandIdlDecompileAccountArgs),
    IdlDecompileExecution(ToolboxCliCommandIdlDecompileExecutionArgs),
    IdlProcessInstruction(ToolboxCliCommandIdlProcessInstructionArgs),
    IdlDescribe(ToolboxCliCommandIdlDescribeArgs),
    InspectAccount(ToolboxCliCommandInspectAccountArgs),
    SearchAddresses(ToolboxCliCommandSearchAddressesArgs),
    SearchSignaturesJson(ToolboxCliCommandSearchSignaturesArgs),
}

impl ToolboxCliCommand {
    pub async fn process(
        &self,
        endpoint: &mut ToolboxEndpoint,
    ) -> Result<(), ToolboxCliError> {
        match self {
            ToolboxCliCommand::GetAccount(args) => args.process(endpoint).await,
            ToolboxCliCommand::GetExecution(args) => {
                args.process(endpoint).await
            },
            ToolboxCliCommand::IdlDecompileAccount(args) => {
                args.process(endpoint).await
            },
            ToolboxCliCommand::IdlDecompileExecution(args) => {
                args.process(endpoint).await
            },
            ToolboxCliCommand::IdlDescribe(args) => {
                args.process(endpoint).await
            },
            ToolboxCliCommand::IdlProcessInstruction(args) => {
                args.process(endpoint).await
            },
            ToolboxCliCommand::InspectAccount(args) => {
                args.process(endpoint).await
            },
            ToolboxCliCommand::SearchAddresses(args) => {
                args.process(endpoint).await
            },
            ToolboxCliCommand::SearchSignaturesJson(args) => {
                args.process(endpoint).await
            },
        }
    }
}
