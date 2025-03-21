mod toolbox_cli;
mod toolbox_cli_command_dev_account;
mod toolbox_cli_command_idl_account;
mod toolbox_cli_command_idl_execution;
mod toolbox_cli_command_idl_instruction;
mod toolbox_cli_command_idl_process;
mod toolbox_cli_command_idl_program;
mod toolbox_cli_command_raw_get_account;
mod toolbox_cli_command_raw_get_execution;
mod toolbox_cli_command_raw_search_addresses;
mod toolbox_cli_command_raw_search_signatures;
mod toolbox_cli_config;
mod toolbox_cli_error;

use clap::Parser;

use crate::toolbox_cli::ToolboxCliArgs;
use crate::toolbox_cli_error::ToolboxCliError;

#[tokio::main]
async fn main() -> Result<(), ToolboxCliError> {
    ToolboxCliArgs::parse().process().await
}
