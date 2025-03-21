use clap::Parser;
use clap::Subcommand;
use solana_cli_config::Config;
use solana_cli_config::CONFIG_FILE;

use crate::toolbox_cli_command_dev_account::ToolboxCliCommandDevAccountArgs;
use crate::toolbox_cli_command_idl_account::ToolboxCliCommandIdlAccountArgs;
use crate::toolbox_cli_command_idl_execution::ToolboxCliCommandIdlExecutionArgs;
use crate::toolbox_cli_command_idl_instruction::ToolboxCliCommandIdlInstructionArgs;
use crate::toolbox_cli_command_idl_process::ToolboxCliCommandIdlProcessArgs;
use crate::toolbox_cli_command_idl_program::ToolboxCliCommandIdlProgramArgs;
use crate::toolbox_cli_command_raw_get_account::ToolboxCliCommandRawGetAccountArgs;
use crate::toolbox_cli_command_raw_get_execution::ToolboxCliCommandRawGetExecutionArgs;
use crate::toolbox_cli_command_raw_search_addresses::ToolboxCliCommandRawSearchAddressesArgs;
use crate::toolbox_cli_command_raw_search_signatures::ToolboxCliCommandRawSearchSignaturesArgs;
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
        value_name = "URL_OR_MONIKER",
        help = "The solana RPC endpoint used"
    )]
    rpc: Option<String>,
    #[arg(
        long,
        value_name = "LEVEL",
        help = "Commitment level used for RPC endpoint"
    )]
    commitment: Option<String>,
    #[arg(
        long,
        value_name = "KEYPAIR_FILE_PATH",
        help = "Keypair used as default payer and 'WALLET' account key"
    )]
    wallet: Option<String>,
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
        if let Some(rpc) = &self.rpc {
            solana_cli_config.json_rpc_url = rpc.to_string();
        }
        if let Some(commitment) = &self.commitment {
            solana_cli_config.commitment = commitment.to_string();
        }
        if let Some(wallet) = &self.wallet {
            solana_cli_config.keypair_path = wallet.to_string();
        }
        self.command
            .process(&ToolboxCliConfig::new(
                solana_cli_config.json_rpc_url,
                solana_cli_config.commitment,
                solana_cli_config.keypair_path,
            ))
            .await
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum ToolboxCliCommand {
    DevAccount(ToolboxCliCommandDevAccountArgs),
    IdlAccount(ToolboxCliCommandIdlAccountArgs),
    IdlExecution(ToolboxCliCommandIdlExecutionArgs),
    IdlInstruction(ToolboxCliCommandIdlInstructionArgs),
    IdlProgram(ToolboxCliCommandIdlProgramArgs),
    IdlProcess(ToolboxCliCommandIdlProcessArgs),
    RawGetAccount(ToolboxCliCommandRawGetAccountArgs),
    RawGetExecution(ToolboxCliCommandRawGetExecutionArgs),
    RawSearchAddresses(ToolboxCliCommandRawSearchAddressesArgs),
    RawSearchSignatures(ToolboxCliCommandRawSearchSignaturesArgs),
}

// TODO - some type of lookup system for addresses by name or smthg

impl ToolboxCliCommand {
    pub async fn process(
        &self,
        config: &ToolboxCliConfig,
    ) -> Result<(), ToolboxCliError> {
        match self {
            ToolboxCliCommand::DevAccount(args) => args.process(config).await,
            ToolboxCliCommand::IdlAccount(args) => args.process(config).await,
            ToolboxCliCommand::IdlExecution(args) => args.process(config).await,
            ToolboxCliCommand::IdlInstruction(args) => {
                args.process(config).await
            },
            ToolboxCliCommand::IdlProgram(args) => args.process(config).await,
            ToolboxCliCommand::IdlProcess(args) => args.process(config).await,
            ToolboxCliCommand::RawGetAccount(args) => {
                args.process(config).await
            },
            ToolboxCliCommand::RawGetExecution(args) => {
                args.process(config).await
            },
            ToolboxCliCommand::RawSearchAddresses(args) => {
                args.process(config).await
            },
            ToolboxCliCommand::RawSearchSignatures(args) => {
                args.process(config).await
            },
        }
    }
}
