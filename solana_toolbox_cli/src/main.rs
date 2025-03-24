mod toolbox_cli;
mod toolbox_cli_command_account;
mod toolbox_cli_command_execution;
mod toolbox_cli_command_history;
mod toolbox_cli_command_instruction;
mod toolbox_cli_command_program;
mod toolbox_cli_command_search;
mod toolbox_cli_config;
mod toolbox_cli_error;
mod toolbox_cli_key;

use clap::Parser;

use crate::toolbox_cli::ToolboxCliArgs;
use crate::toolbox_cli_error::ToolboxCliError;

#[tokio::main]
async fn main() -> Result<(), ToolboxCliError> {
    ToolboxCliArgs::parse().process().await
}
