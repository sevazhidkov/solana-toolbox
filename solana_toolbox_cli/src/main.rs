mod toolbox_cli;
mod toolbox_cli_command_account;
mod toolbox_cli_command_execution;
mod toolbox_cli_command_find;
mod toolbox_cli_command_history;
mod toolbox_cli_command_instruction;
mod toolbox_cli_command_program;
mod toolbox_cli_context;
mod toolbox_cli_key;

use anyhow::Result;
use clap::Parser;

use crate::toolbox_cli::ToolboxCliArgs;

#[tokio::main]
async fn main() -> Result<()> {
    ToolboxCliArgs::parse().process().await
}
