use clap::Parser;
use clap::Subcommand;
use serde_json::Value;
use solana_cli_config::Config;
use solana_cli_config::CONFIG_FILE;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_cli_command_account::ToolboxCliCommandAccountArgs;
use crate::toolbox_cli_command_execution::ToolboxCliCommandExecutionArgs;
use crate::toolbox_cli_command_find::ToolboxCliCommandFindArgs;
use crate::toolbox_cli_command_history::ToolboxCliCommandHistoryArgs;
use crate::toolbox_cli_command_instruction::ToolboxCliCommandInstructionArgs;
use crate::toolbox_cli_command_program::ToolboxCliCommandProgramArgs;
use crate::toolbox_cli_context::ToolboxCliContext;
use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Parser)]
#[command(version, about = "Tooling to interact with a solana endpoint")]
pub struct ToolboxCliArgs {
    #[arg(
        long = "config",
        value_name = "CONFIG_FILE_PATH",
        help = "To use a different path for the solana's config YAML file"
    )]
    config: Option<String>,
    #[arg(
        short,
        long = "url",
        alias = "rpc",
        value_name = "URL_OR_MONIKER",
        help = "The solana RPC endpoint's URL used"
    )]
    url: Option<String>,
    #[arg(
        long = "commitment",
        value_name = "COMMITMENT_LEVEL",
        help = "Commitment level used for RPC endpoint"
    )]
    commitment: Option<String>,
    #[arg(
        short,
        long = "keypair",
        alias = "wallet",
        value_name = "KEYPAIR_FILE_PATH",
        help = "Keypair used as transaction payer"
    )]
    keypair: Option<String>,
    #[arg(
        short,
        long = "idl",
        alias = "idls",
        value_name = "PROGRAM_ID:IDL_FILE_PATH",
        help = "Use a custom IDL file for a specific Program ID"
    )]
    idls: Vec<String>,
    #[arg(long = "compact", help = "Output compacted JSON")]
    compact: bool,
    #[command(subcommand)]
    command: ToolboxCliCommand,
}

impl ToolboxCliArgs {
    pub async fn process(&self) -> Result<(), ToolboxCliError> {
        let mut solana_cli_config = Config::load(
            self.config
                .as_ref()
                .or(CONFIG_FILE.as_ref())
                .ok_or_else(|| {
                    ToolboxCliError::Custom(
                        "Could not find solana config file".to_string(),
                    )
                })?,
        )?;
        if let Some(commitment) = &self.commitment {
            solana_cli_config.commitment = commitment.to_string();
        }
        if let Some(url) = &self.url {
            solana_cli_config.json_rpc_url =
                ToolboxEndpoint::get_url_from_url_or_moniker(url).to_string();
        }
        if let Some(keypair) = &self.keypair {
            solana_cli_config.keypair_path = keypair.to_string();
        }
        let context = ToolboxCliContext::new(
            solana_cli_config.json_rpc_url,
            solana_cli_config.commitment,
            solana_cli_config.keypair_path,
            self.idls.clone(),
        );
        let json = self.command.process(&context).await?;
        if self.compact {
            println!("{}", serde_json::to_string(&json)?);
        } else {
            println!("{}", serde_json::to_string_pretty(&json)?);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum ToolboxCliCommand {
    Account(ToolboxCliCommandAccountArgs),
    Execution(ToolboxCliCommandExecutionArgs),
    Find(ToolboxCliCommandFindArgs),
    History(ToolboxCliCommandHistoryArgs),
    Instruction(ToolboxCliCommandInstructionArgs),
    Program(ToolboxCliCommandProgramArgs),
}

// TODO (MEDIUM) - some type of lookup system for addresses by name or smthg

impl ToolboxCliCommand {
    pub async fn process(
        &self,
        context: &ToolboxCliContext,
    ) -> Result<Value, ToolboxCliError> {
        match self {
            ToolboxCliCommand::Account(args) => args.process(context).await,
            ToolboxCliCommand::Execution(args) => args.process(context).await,
            ToolboxCliCommand::Find(args) => args.process(context).await,
            ToolboxCliCommand::History(args) => args.process(context).await,
            ToolboxCliCommand::Instruction(args) => args.process(context).await,
            ToolboxCliCommand::Program(args) => args.process(context).await,
        }
    }
}
