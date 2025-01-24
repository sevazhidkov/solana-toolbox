use std::collections::HashMap;

use serde_json::Map;
use serde_json::Value;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_ok_or_else;

impl ToolboxIdl {
    pub async fn resolve_instruction(
        &self,
        endpoint: &mut ToolboxEndpoint,
        program_id: &Pubkey,
        instruction_name: &str,
        instruction_accounts_addresses: &HashMap<String, Pubkey>,
        instruction_args: &Map<String, Value>,
    ) -> Result<Instruction, ToolboxIdlError> {
        let mut instruction_accounts_addresses =
            instruction_accounts_addresses.clone();
        let mut instruction_accounts_values = self
            .get_accounts_values_by_name(
                endpoint,
                &instruction_accounts_addresses,
            )
            .await?;
        let instruction_accounts_names =
            self.get_instruction_accounts_names(instruction_name)?;
        loop {
            let mut made_progress = false;
            for instruction_account_name in &instruction_accounts_names {
                if instruction_accounts_addresses
                    .contains_key(instruction_account_name)
                {
                    continue;
                }
                if let Ok(instruction_account_address) = self
                    .resolve_instruction_account_address(
                        instruction_account_name,
                        program_id,
                        instruction_name,
                        &instruction_accounts_addresses,
                        &instruction_accounts_values,
                        instruction_args,
                    )
                {
                    made_progress = true;
                    instruction_accounts_addresses.insert(
                        instruction_account_name.to_string(),
                        instruction_account_address,
                    );
                    if let Ok(Some(instruction_account_value)) = self
                        .get_account_value(
                            endpoint,
                            &instruction_account_address,
                        )
                        .await
                    {
                        instruction_accounts_values.insert(
                            instruction_account_name.to_string(),
                            instruction_account_value,
                        );
                    }
                }
            }
            if !made_progress {
                break;
            }
        }
        self.generate_instruction(
            program_id,
            instruction_name,
            &instruction_accounts_addresses,
            instruction_args,
        )
    }

    pub fn generate_instruction(
        &self,
        program_id: &Pubkey,
        instruction_name: &str,
        instruction_accounts_addresses: &HashMap<String, Pubkey>,
        instruction_args: &Map<String, Value>,
    ) -> Result<Instruction, ToolboxIdlError> {
        let instruction_accounts = self.generate_instruction_accounts(
            instruction_name,
            instruction_accounts_addresses,
        )?;
        let instruction_data =
            self.compile_instruction_data(instruction_name, instruction_args)?;
        Ok(Instruction {
            program_id: *program_id,
            accounts: instruction_accounts,
            data: instruction_data,
        })
    }

    pub fn parse_instruction(
        &self,
        instruction: &Instruction,
    ) -> Result<(HashMap<String, Pubkey>, Map<String, Value>), ToolboxIdlError>
    {
        let instruction_name = idl_ok_or_else(
            self.guess_instruction_name(&instruction.data),
            "Could not guess instruction name",
            &ToolboxIdlBreadcrumbs::default().as_val("instruction_name"),
        )?;
        let instruction_accounts_addresses = self
            .decompile_instruction_accounts_addresses(
                instruction_name,
                instruction,
            )?;
        let instruction_args = self
            .decompile_instruction_data(instruction_name, &instruction.data)?;
        Ok((instruction_accounts_addresses, instruction_args))
    }
}
