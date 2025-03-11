use clap::Parser;
use clap::Subcommand;
use solana_cli_config::Config;
use solana_cli_config::CONFIG_FILE;

use crate::toolbox_cli_command_get_account::ToolboxCliCommandGetAccountArgs;
use crate::toolbox_cli_command_get_execution::ToolboxCliCommandGetExecutionArgs;
use crate::toolbox_cli_command_idl_decompile_account::ToolboxCliCommandIdlDecompileAccountArgs;
use crate::toolbox_cli_command_idl_decompile_execution::ToolboxCliCommandIdlDecompileExecutionArgs;
use crate::toolbox_cli_command_idl_describe::ToolboxCliCommandIdlDescribeArgs;
use crate::toolbox_cli_command_idl_process_instruction::ToolboxCliCommandIdlProcessInstructionArgs;
use crate::toolbox_cli_command_idl_resolve_instruction_accounts::ToolboxCliCommandIdlResolveInstructionAccountsArgs;
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
        self.command.process(&config).await
    }
}

// TODO - command to download JSON IDL
#[derive(Debug, Clone, Subcommand)]
pub enum ToolboxCliCommand {
    GetAccount(ToolboxCliCommandGetAccountArgs),
    GetExecution(ToolboxCliCommandGetExecutionArgs),
    IdlDecompileAccount(ToolboxCliCommandIdlDecompileAccountArgs),
    IdlDecompileExecution(ToolboxCliCommandIdlDecompileExecutionArgs),
    IdlDescribe(ToolboxCliCommandIdlDescribeArgs),
    IdlProcessInstruction(ToolboxCliCommandIdlProcessInstructionArgs),
    IdlResolveInstructionAccounts(
        ToolboxCliCommandIdlResolveInstructionAccountsArgs,
    ),
    InspectAccount(ToolboxCliCommandInspectAccountArgs),
    SearchAddresses(ToolboxCliCommandSearchAddressesArgs),
    SearchSignaturesJson(ToolboxCliCommandSearchSignaturesArgs),
}

impl ToolboxCliCommand {
    pub async fn process(
        &self,
        config: &Config,
    ) -> Result<(), ToolboxCliError> {
        match self {
            ToolboxCliCommand::GetAccount(args) => args.process(config).await,
            ToolboxCliCommand::GetExecution(args) => args.process(config).await,
            ToolboxCliCommand::IdlDecompileAccount(args) => {
                args.process(config).await
            },
            ToolboxCliCommand::IdlDecompileExecution(args) => {
                args.process(config).await
            },
            ToolboxCliCommand::IdlDescribe(args) => args.process(config).await,
            ToolboxCliCommand::IdlProcessInstruction(args) => {
                args.process(config).await
            },
            ToolboxCliCommand::IdlResolveInstructionAccounts(args) => {
                args.process(config).await
            },
            ToolboxCliCommand::InspectAccount(args) => {
                args.process(config).await
            },
            ToolboxCliCommand::SearchAddresses(args) => {
                args.process(config).await
            },
            ToolboxCliCommand::SearchSignaturesJson(args) => {
                args.process(config).await
            },
        }
    }
}
