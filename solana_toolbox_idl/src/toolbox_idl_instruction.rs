use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccount;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFields;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;
use crate::toolbox_idl_utils::idl_convert_to_value_name;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlInstruction {
    pub name: String,
    pub docs: Option<Value>,
    pub discriminator: Vec<u8>,
    pub accounts: Vec<ToolboxIdlInstructionAccount>,
    pub args_type_flat_fields: ToolboxIdlTypeFlatFields,
    pub args_type_full_fields: Arc<ToolboxIdlTypeFullFields>,
    // TODO - support "discriminant" as a type/value const ?
}

impl Default for ToolboxIdlInstruction {
    fn default() -> ToolboxIdlInstruction {
        ToolboxIdlInstruction {
            name: ToolboxIdlInstruction::sanitize_name("unknown_instruction"),
            docs: None,
            discriminator: vec![],
            accounts: vec![],
            args_type_flat_fields: ToolboxIdlTypeFlatFields::None,
            args_type_full_fields: ToolboxIdlTypeFullFields::None.into(),
        }
    }
}

impl ToolboxIdlInstruction {
    pub fn sanitize_name(name: &str) -> String {
        idl_convert_to_value_name(name)
    }

    pub fn compile(
        &self,
        program_id: &Pubkey,
        instruction_payload: &Value,
        instruction_addresses: &HashMap<String, Pubkey>,
    ) -> Result<Instruction, ToolboxIdlError> {
        Ok(Instruction {
            program_id: *program_id,
            data: self.compile_payload(instruction_payload)?,
            accounts: self.compile_addresses(instruction_addresses)?,
        })
    }

    pub fn decompile(
        &self,
        instruction: &Instruction,
    ) -> Result<(Pubkey, Value, HashMap<String, Pubkey>), ToolboxIdlError> {
        Ok((
            instruction.program_id,
            self.decompile_payload(&instruction.data)?,
            self.decompile_addresses(&instruction.accounts)?,
        ))
    }

    pub fn find_addresses(
        &self,
        program_id: &Pubkey,
        instruction_payload: &Value,
        instruction_addresses: &HashMap<String, Pubkey>,
    ) -> HashMap<String, Pubkey> {
        self.find_addresses_with_accounts_content_types_and_states(
            program_id,
            instruction_payload,
            instruction_addresses,
            &HashMap::new(),
        )
    }

    pub fn find_addresses_with_accounts_content_types_and_states(
        &self,
        program_id: &Pubkey,
        instruction_payload: &Value,
        instruction_addresses: &HashMap<String, Pubkey>,
        accounts_content_types_and_states: &HashMap<
            String,
            (Arc<ToolboxIdlTypeFull>, Value),
        >,
    ) -> HashMap<String, Pubkey> {
        let mut instruction_addresses = instruction_addresses.clone();
        loop {
            let breadcrumbs = ToolboxIdlBreadcrumbs::default();
            let mut made_progress = false;
            for instruction_account in &self.accounts {
                if instruction_addresses.contains_key(&instruction_account.name)
                {
                    continue;
                }
                if let Ok(instruction_address) = instruction_account.try_find(
                    program_id,
                    &self.args_type_full_fields,
                    instruction_payload,
                    &instruction_addresses,
                    accounts_content_types_and_states,
                    &breadcrumbs.with_idl(&instruction_account.name),
                ) {
                    made_progress = true;
                    instruction_addresses.insert(
                        instruction_account.name.to_string(),
                        instruction_address,
                    );
                }
            }
            if !made_progress {
                break;
            }
        }
        instruction_addresses
    }

    pub fn get_addresses_dependencies(&self) -> HashMap<String, String> {
        let mut dependencies = HashMap::new();
        for account in &self.accounts {
            if let Some(account_address) = &account.address {
                dependencies.insert(
                    account.name.to_string(),
                    format!("={}", account_address),
                );
            } else if let Some(account_pda) = &account.pda {
                let mut dependencies_blobs = vec![];
                for account_pda_seed in &account_pda.seeds {
                    if let Some((kind, path)) = account_pda_seed.info() {
                        dependencies_blobs.push(format!("{}s.{}", kind, path));
                    }
                }
                if let Some(account_pda_program) = &account_pda.program {
                    if let Some((kind, path)) = account_pda_program.info() {
                        dependencies_blobs.push(format!("{}s.{}", kind, path));
                    }
                }
                dependencies.insert(
                    account.name.to_string(),
                    format!("[{}]", dependencies_blobs.join(",")),
                );
            } else {
                dependencies.insert(
                    account.name.to_string(),
                    "MUST_BE_SPECIFIED".to_string(),
                );
            }
        }
        dependencies
    }
}
