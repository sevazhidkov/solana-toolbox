use clap::Parser;
use clap::Subcommand;
use solana_cli_config::Config;
use solana_cli_config::CONFIG_FILE;

use crate::toolbox_cli_command_get_account::ToolboxCliCommandGetAccountArgs;
use crate::toolbox_cli_command_get_execution::ToolboxCliCommandGetExecutionArgs;
use crate::toolbox_cli_command_idl_process_instruction::ToolboxCliCommandIdlProcessInstructionArgs;
use crate::toolbox_cli_command_idl_resolve_account::ToolboxCliCommandIdlResolveAccountArgs;
use crate::toolbox_cli_command_idl_resolve_execution::ToolboxCliCommandIdlResolveExecutionArgs;
use crate::toolbox_cli_command_idl_resolve_instruction_addresses::ToolboxCliCommandIdlResolveInstructionAddressesArgs;
use crate::toolbox_cli_command_idl_resolve_program::ToolboxCliCommandIdlResolveProgramArgs;
use crate::toolbox_cli_command_inspect_account::ToolboxCliCommandInspectAccountArgs;
use crate::toolbox_cli_command_search_addresses::ToolboxCliCommandSearchAddressesArgs;
use crate::toolbox_cli_command_search_signatures::ToolboxCliCommandSearchSignaturesArgs;
use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Parser)]
pub struct ToolboxCliArgs {
    #[arg(short, long)]
    config: Option<String>,
    #[command(subcommand)]
    command: ToolboxCliCommand,
}

impl ToolboxCliArgs {
    pub async fn process(&self) -> Result<(), ToolboxCliError> {
        let config = Config::load(
            self.config
                .as_ref()
                .or(CONFIG_FILE.as_ref())
                .ok_or_else(|| {
                    ToolboxCliError::Custom(
                        "Could not find solana config file".to_string(),
                    )
                })?,
        )?;
        // TODO - custom url/wallet
        self.command.process(&config).await
    }
}

// TODO - command to generate IX data for DAO use ?
// TODO - command to download JSON IDL
#[derive(Debug, Clone, Subcommand)]
pub enum ToolboxCliCommand {
    GetAccount(ToolboxCliCommandGetAccountArgs),
    GetExecution(ToolboxCliCommandGetExecutionArgs),
    IdlProcessInstruction(ToolboxCliCommandIdlProcessInstructionArgs),
    IdlResolveAccount(ToolboxCliCommandIdlResolveAccountArgs),
    IdlResolveExecution(ToolboxCliCommandIdlResolveExecutionArgs),
    IdlResolveProgram(ToolboxCliCommandIdlResolveProgramArgs),
    IdlResolveInstructionAddresses(
        ToolboxCliCommandIdlResolveInstructionAddressesArgs,
    ),
    InspectAccount(ToolboxCliCommandInspectAccountArgs),
    SearchAddresses(ToolboxCliCommandSearchAddressesArgs),
    SearchSignatures(ToolboxCliCommandSearchSignaturesArgs),
}

impl ToolboxCliCommand {
    pub async fn process(
        &self,
        config: &Config,
    ) -> Result<(), ToolboxCliError> {
        match self {
            ToolboxCliCommand::GetAccount(args) => args.process(config).await,
            ToolboxCliCommand::GetExecution(args) => args.process(config).await,
            ToolboxCliCommand::IdlResolveAccount(args) => {
                args.process(config).await
            },
            ToolboxCliCommand::IdlResolveExecution(args) => {
                args.process(config).await
            },
            ToolboxCliCommand::IdlResolveProgram(args) => {
                args.process(config).await
            },
            ToolboxCliCommand::IdlProcessInstruction(args) => {
                args.process(config).await
            },
            ToolboxCliCommand::IdlResolveInstructionAddresses(args) => {
                args.process(config).await
            },
            ToolboxCliCommand::InspectAccount(args) => {
                args.process(config).await
            },
            ToolboxCliCommand::SearchAddresses(args) => {
                args.process(config).await
            },
            ToolboxCliCommand::SearchSignatures(args) => {
                args.process(config).await
            },
        }
    }
}
