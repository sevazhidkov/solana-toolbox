mod toolbox_cli;
mod toolbox_cli_account;
mod toolbox_cli_account_inspect;
mod toolbox_cli_account_state;
mod toolbox_cli_error;
mod toolbox_cli_idl;

use clap::Parser;

use crate::toolbox_cli::ToolboxCliArgs;
use crate::toolbox_cli_error::ToolboxCliError;

#[tokio::main]
async fn main() -> Result<(), ToolboxCliError> {
    ToolboxCliArgs::parse().process().await
}
