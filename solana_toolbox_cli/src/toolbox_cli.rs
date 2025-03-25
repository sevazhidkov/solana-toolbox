use clap::Parser;
use clap::Subcommand;
use solana_cli_config::Config;
use solana_cli_config::CONFIG_FILE;

use crate::toolbox_cli_command_account::ToolboxCliCommandAccountArgs;
use crate::toolbox_cli_command_execution::ToolboxCliCommandExecutionArgs;
use crate::toolbox_cli_command_find::ToolboxCliCommandFindArgs;
use crate::toolbox_cli_command_history::ToolboxCliCommandHistoryArgs;
use crate::toolbox_cli_command_instruction::ToolboxCliCommandInstructionArgs;
use crate::toolbox_cli_command_program::ToolboxCliCommandProgramArgs;
use crate::toolbox_cli_config::ToolboxCliConfig;
use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Parser)]
#[command(version, about = "Tooling to interact with a solana endpoint")]
pub struct ToolboxCliArgs {
    #[arg(
        long,
        value_name = "CONFIG_FILE_PATH",
        help = "To use a different path for the solana's config YAML file"
    )]
    config: Option<String>,
    #[arg(
        long,
        value_name = "COMMITMENT_LEVEL",
        help = "Commitment level used for RPC endpoint"
    )]
    commitment: Option<String>,
    #[arg(
        short,
        long,
        alias = "rpc",
        value_name = "URL_OR_MONIKER",
        help = "The solana RPC endpoint's URL used"
    )]
    url: Option<String>,
    #[arg(
        short,
        long,
        alias = "wallet",
        value_name = "KEYPAIR_FILE_PATH",
        help = "Keypair used as payer and 'KEYPAIR' account key"
    )]
    keypair: Option<String>,
    #[arg(
        short,
        long,
        value_delimiter = ',',
        value_name = "PROGRAM_ID:IDL_FILE_PATH",
        help = "Use custom IDLs for programs, format: [ProgramId:IdlFile]"
    )]
    idls: Vec<String>,
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
            solana_cli_config.json_rpc_url = url.to_string();
        }
        if let Some(keypair) = &self.keypair {
            solana_cli_config.keypair_path = keypair.to_string();
        }
        self.command
            .process(&ToolboxCliConfig::new(
                solana_cli_config.json_rpc_url,
                solana_cli_config.commitment,
                solana_cli_config.keypair_path,
                self.idls.clone(),
            ))
            .await
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum ToolboxCliCommand {
    Account(ToolboxCliCommandAccountArgs),
    Execution(ToolboxCliCommandExecutionArgs),
    History(ToolboxCliCommandHistoryArgs),
    Instruction(ToolboxCliCommandInstructionArgs),
    Program(ToolboxCliCommandProgramArgs),
    Find(ToolboxCliCommandFindArgs),
}

// TODO - some type of lookup system for addresses by name or smthg

impl ToolboxCliCommand {
    pub async fn process(
        &self,
        config: &ToolboxCliConfig,
    ) -> Result<(), ToolboxCliError> {
        match self {
            ToolboxCliCommand::Account(args) => args.process(config).await,
            ToolboxCliCommand::Execution(args) => args.process(config).await,
            ToolboxCliCommand::History(args) => args.process(config).await,
            ToolboxCliCommand::Instruction(args) => args.process(config).await,
            ToolboxCliCommand::Program(args) => args.process(config).await,
            ToolboxCliCommand::Find(args) => args.process(config).await,
        }
    }
}
