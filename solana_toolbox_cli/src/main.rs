mod toolbox_cli;
mod toolbox_cli_command_get_account_json;
mod toolbox_cli_command_get_execution_json;
mod toolbox_cli_command_idl_decompiled_account_json;
mod toolbox_cli_command_idl_decompiled_execution_json;
mod toolbox_cli_command_inspect_account;
mod toolbox_cli_command_search_addresses_json;
mod toolbox_cli_command_search_signatures_json;
mod toolbox_cli_error;

use clap::Parser;

use crate::toolbox_cli::ToolboxCliArgs;
use crate::toolbox_cli_error::ToolboxCliError;

#[tokio::main]
async fn main() -> Result<(), ToolboxCliError> {
    ToolboxCliArgs::parse().process().await
}
