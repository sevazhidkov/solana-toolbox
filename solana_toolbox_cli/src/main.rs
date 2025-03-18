mod toolbox_cli;
mod toolbox_cli_command_dev_inspect_account;
mod toolbox_cli_command_idl_process_instruction;
mod toolbox_cli_command_idl_resolve_account;
mod toolbox_cli_command_idl_resolve_execution;
mod toolbox_cli_command_idl_resolve_instruction_addresses;
mod toolbox_cli_command_idl_resolve_instruction_base58;
mod toolbox_cli_command_idl_resolve_program;
mod toolbox_cli_command_raw_get_account;
mod toolbox_cli_command_raw_get_execution;
mod toolbox_cli_command_raw_search_addresses;
mod toolbox_cli_command_raw_search_signatures;
mod toolbox_cli_error;
mod toolbox_cli_utils;

use clap::Parser;

use crate::toolbox_cli::ToolboxCliArgs;
use crate::toolbox_cli_error::ToolboxCliError;

#[tokio::main]
async fn main() -> Result<(), ToolboxCliError> {
    ToolboxCliArgs::parse().process().await
}
