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
            .instructions
            .values()
            .map(|instruction| {
                let mut instruction_accounts_resolvables = vec![];
                let mut instruction_accounts_unresolvables = vec![];
                let mut instruction_accounts_signers = vec![];
                for instruction_account in &instruction.accounts {
                    if instruction_account.pda.is_none()
                        && instruction_account.address.is_none()
                    {
                        instruction_accounts_unresolvables
                            .push(instruction_account.name.to_string());
                    } else {
                        instruction_accounts_resolvables
                            .push(instruction_account.name.to_string());
                    }
                    if instruction_account.is_signer {
                        instruction_accounts_signers
                            .push(instruction_account.name.to_string());
                    }
                }
                (
                    instruction.name.to_string(),
                    json!({
                        "args": instruction.data_type_flat.describe(),
                        "accounts": instruction_accounts_unresolvables,
                        "signers": instruction_accounts_signers,
                        "resolvables": instruction_accounts_resolvables,
                    }),
                )
            })
            .collect::<Vec<_>>();
        let json_accounts = idl
            .accounts
            .values()
            .map(|account| {
                (
                    account.name.to_string(),
                    json!(account.data_type_flat.describe()),
                )
            })
            .collect::<Vec<_>>();
        let json_types = idl
            .typedefs
            .values()
            .map(|typedef| {
                (
                    typedef.name.to_string(),
                    json!(typedef.type_flat.describe()),
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
