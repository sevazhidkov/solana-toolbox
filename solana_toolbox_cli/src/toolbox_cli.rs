use clap::Parser;
use clap::Subcommand;
use solana_cli_config::Config;
use solana_cli_config::CONFIG_FILE;

use crate::toolbox_cli_command_dev_inspect_account::ToolboxCliCommandDevInspectAccountArgs;
use crate::toolbox_cli_command_idl_process_instruction::ToolboxCliCommandIdlProcessInstructionArgs;
use crate::toolbox_cli_command_idl_resolve_account::ToolboxCliCommandIdlResolveAccountArgs;
use crate::toolbox_cli_command_idl_resolve_execution::ToolboxCliCommandIdlResolveExecutionArgs;
use crate::toolbox_cli_command_idl_resolve_instruction_addresses::ToolboxCliCommandIdlResolveInstructionAddressesArgs;
use crate::toolbox_cli_command_idl_resolve_instruction_base58::ToolboxCliCommandIdlResolveInstructionBase58Args;
use crate::toolbox_cli_command_idl_resolve_program::ToolboxCliCommandIdlResolveProgramArgs;
use crate::toolbox_cli_command_raw_get_account::ToolboxCliCommandRawGetAccountArgs;
use crate::toolbox_cli_command_raw_get_execution::ToolboxCliCommandRawGetExecutionArgs;
use crate::toolbox_cli_command_raw_search_addresses::ToolboxCliCommandRawSearchAddressesArgs;
use crate::toolbox_cli_command_raw_search_occurrences::ToolboxCliCommandRawSearchOccurrencesArgs;
use crate::toolbox_cli_command_raw_search_signatures::ToolboxCliCommandRawSearchSignaturesArgs;
use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Parser)]
pub struct ToolboxCliArgs {
    #[arg(short, long)]
    rpc: Option<String>,
    #[arg(short, long)]
    keypair: Option<String>,
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
        if let Some(rpc) = self.rpc {}
        // TODO - custom url/wallet
        self.command.process(&config).await
    }
}

// TODO - command to generate IX data for DAO use ?
// TODO - command to download JSON IDL
#[derive(Debug, Clone, Subcommand)]
pub enum ToolboxCliCommand {
    DevInspectAccount(ToolboxCliCommandDevInspectAccountArgs),
    IdlProcessInstruction(ToolboxCliCommandIdlProcessInstructionArgs),
    IdlResolveAccount(ToolboxCliCommandIdlResolveAccountArgs),
    IdlResolveExecution(ToolboxCliCommandIdlResolveExecutionArgs),
    IdlResolveProgram(ToolboxCliCommandIdlResolveProgramArgs),
    IdlResolveInstructionAddresses(
        ToolboxCliCommandIdlResolveInstructionAddressesArgs,
    ),
    IdlResolveInstructionBase58(
        ToolboxCliCommandIdlResolveInstructionBase58Args,
    ),
    RawGetAccount(ToolboxCliCommandRawGetAccountArgs),
    RawGetExecution(ToolboxCliCommandRawGetExecutionArgs),
    RawSearchAddresses(ToolboxCliCommandRawSearchAddressesArgs),
    RawSearchOccurrences(ToolboxCliCommandRawSearchOccurrencesArgs),
    RawSearchSignatures(ToolboxCliCommandRawSearchSignaturesArgs),
}

impl ToolboxCliCommand {
    pub async fn process(
        &self,
        config: &Config,
    ) -> Result<(), ToolboxCliError> {
        match self {
            ToolboxCliCommand::DevInspectAccount(args) => {
                args.process(config).await
            },
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
            ToolboxCliCommand::IdlResolveInstructionBase58(args) => {
                args.process(config).await
            },
            ToolboxCliCommand::RawGetAccount(args) => {
                args.process(config).await
            },
            ToolboxCliCommand::RawGetExecution(args) => {
                args.process(config).await
            },
            ToolboxCliCommand::RawSearchAddresses(args) => {
                args.process(config).await
            },
            ToolboxCliCommand::RawSearchOccurrences(args) => {
                args.process(config).await
            },
            ToolboxCliCommand::RawSearchSignatures(args) => {
                args.process(config).await
            },
        }
    }
}
