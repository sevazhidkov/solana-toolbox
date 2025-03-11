use std::str::FromStr;

use clap::Args;
use serde_json::json;
use serde_json::Map;
use serde_json::Value;
use solana_cli_config::Config;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdl;

use crate::toolbox_cli_error::ToolboxCliError;
use crate::toolbox_cli_utils::ToolboxCliUtils;

#[derive(Debug, Clone, Args)]
pub struct ToolboxCliCommandIdlDescribeArgs {
    program_address: String,
}

impl ToolboxCliCommandIdlDescribeArgs {
    pub async fn process(
        &self,
        config: &Config,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = ToolboxCliUtils::new_endpoint(config)?;
        let program_address = Pubkey::from_str(&self.program_address).unwrap();
        let idl =
            ToolboxIdl::get_for_program_id(&mut endpoint, &program_address)
                .await?
                .unwrap(); // TODO - handle unwrap
        let json_instructions = idl
            .program_instructions
            .values()
            .map(|program_instruction| {
                let mut program_instruction_accounts_resolvables = vec![];
                let mut program_instruction_accounts_unresolvables = vec![];
                let mut program_instruction_accounts_signers = vec![];
                for program_instruction_account in &program_instruction.accounts
                {
                    if program_instruction_account.pda.is_none()
                        && program_instruction_account.address.is_none()
                    {
                        program_instruction_accounts_unresolvables
                            .push(program_instruction_account.name.to_string());
                    } else {
                        program_instruction_accounts_resolvables
                            .push(program_instruction_account.name.to_string());
                    }
                    if program_instruction_account.is_signer {
                        program_instruction_accounts_signers
                            .push(program_instruction_account.name.to_string());
                    }
                }
                (
                    program_instruction.name.to_string(),
                    json!({
                        "args": program_instruction.data_type_flat.describe(),
                        "accounts": program_instruction_accounts_unresolvables,
                        "signers": program_instruction_accounts_signers,
                        "resolvables": program_instruction_accounts_resolvables,
                    }),
                )
            })
            .collect::<Vec<_>>();
        let json_accounts = idl
            .program_accounts
            .values()
            .map(|program_account| {
                (
                    program_account.name.to_string(),
                    json!(program_account.data_type_flat.describe()),
                )
            })
            .collect::<Vec<_>>();
        let json_types = idl
            .program_typedefs
            .values()
            .map(|program_type| {
                (
                    program_type.name.to_string(),
                    json!(program_type.type_flat.describe()),
                )
            })
            .collect::<Vec<_>>();
        let json = json!({
            "instructions": Value::Object(Map::from_iter(json_instructions)),
            "accounts": Value::Object(Map::from_iter(json_accounts)),
            "types": Value::Object(Map::from_iter(json_types)),
        });
        println!("{}", serde_json::to_string(&json)?);
        Ok(())
    }
}
