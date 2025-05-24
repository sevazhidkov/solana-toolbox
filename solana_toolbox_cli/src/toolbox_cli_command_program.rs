use anyhow::Result;
use clap::Args;
use clap::Subcommand;
use serde_json::json;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdlFormat;

use crate::toolbox_cli_context::ToolboxCliContext;

#[derive(Debug, Clone, Args)]
#[command(about = "Resolve a program's IDL")]
pub struct ToolboxCliCommandProgramArgs {
    #[arg(value_name = "PROGRAM_ID", help = "The Program ID pubkey in base58")]
    program_id: String,
    #[command(subcommand)]
    command: ToolboxCliCommandProgramCommand,
}

impl ToolboxCliCommandProgramArgs {
    pub async fn process(&self, context: &ToolboxCliContext) -> Result<Value> {
        self.command
            .process(context, context.parse_key(&self.program_id)?.address())
            .await
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum ToolboxCliCommandProgramCommand {
    Summary(ToolboxCliCommandProgramCommandSummaryArgs),
    Export(ToolboxCliCommandProgramCommandExportArgs),
}

impl ToolboxCliCommandProgramCommand {
    pub async fn process(
        &self,
        context: &ToolboxCliContext,
        program_id: Pubkey,
    ) -> Result<Value> {
        match self {
            ToolboxCliCommandProgramCommand::Summary(args) => {
                args.process(context, program_id).await
            },
            ToolboxCliCommandProgramCommand::Export(args) => {
                args.process(context, program_id).await
            },
        }
    }
}

#[derive(Debug, Clone, Args)]
#[command(about = "List the program's accounts/instructions definitions")]
pub struct ToolboxCliCommandProgramCommandSummaryArgs {}

impl ToolboxCliCommandProgramCommandSummaryArgs {
    pub async fn process(
        &self,
        context: &ToolboxCliContext,
        program_id: Pubkey,
    ) -> Result<Value> {
        let mut endpoint = context.create_endpoint().await?;
        let mut idl_service = context.create_service().await?;
        let idl_program = idl_service
            .load_program(&mut endpoint, &program_id)
            .await?
            .unwrap_or_default();
        let mut json_accounts_names = vec![];
        for idl_account_name in idl_program.accounts.keys() {
            json_accounts_names.push(json!(idl_account_name));
        }
        let mut json_instructions_names = vec![];
        for idl_account_name in idl_program.instructions.keys() {
            json_instructions_names.push(json!(idl_account_name));
        }
        let mut json_pdas = vec![];
        for idl_instruction in idl_program.instructions.values() {
            for idl_instruction_account in &idl_instruction.accounts {
                if idl_instruction_account.pda.is_some() {
                    json_pdas.push(format!(
                        "{} -> {}",
                        idl_instruction.name, idl_instruction_account.name
                    ));
                }
            }
        }
        Ok(json!({
            "metadata": idl_program.export_metadata(),
            "accounts": json_accounts_names,
            "instructions": json_instructions_names,
            "pdas": json_pdas,
        }))
    }
}

#[derive(Debug, Clone, Args)]
#[command(about = "Resolve, format and display the raw Program's parsed IDL")]
pub struct ToolboxCliCommandProgramCommandExportArgs {
    #[arg(
        default_value = "HUMAN",
        value_name = "FORMAT_DIALECT_NAME",
        help = "The IDL's dialect to export into"
    )]
    format: String,
}

impl ToolboxCliCommandProgramCommandExportArgs {
    pub async fn process(
        &self,
        context: &ToolboxCliContext,
        program_id: Pubkey,
    ) -> Result<Value> {
        let mut endpoint = context.create_endpoint().await?;
        let mut idl_service = context.create_service().await?;
        let idl_program = idl_service
            .load_program(&mut endpoint, &program_id)
            .await?
            .unwrap_or_default();
        let format_str: &str = &self.format.to_ascii_lowercase();
        let format = match format_str {
            "anchor_26" | "anchor_old" => ToolboxIdlFormat::anchor_26(),
            "anchor_30" | "anchor_new" => ToolboxIdlFormat::anchor_30(),
            _ => ToolboxIdlFormat::human(),
        };
        Ok(idl_program.export(&format))
    }
}
