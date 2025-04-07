use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_account::ToolboxIdlAccount;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_instruction::ToolboxIdlInstruction;
use crate::toolbox_idl_typedef::ToolboxIdlTypedef;
use crate::toolbox_idl_utils::idl_convert_to_type_name;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ToolboxIdlProgramMetadata {
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub spec: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ToolboxIdlProgram {
    pub address: Option<Pubkey>,
    pub docs: Option<Value>,
    pub metadata: ToolboxIdlProgramMetadata,
    pub instructions: HashMap<String, Arc<ToolboxIdlInstruction>>,
    pub accounts: HashMap<String, Arc<ToolboxIdlAccount>>,
    pub typedefs: HashMap<String, Arc<ToolboxIdlTypedef>>,
    pub errors: HashMap<String, Arc<ToolboxIdlError>>,
}

impl ToolboxIdlProgram {
    pub fn sanitize_name(name: &str) -> String {
        idl_convert_to_type_name(name)
    }

    pub fn guess_account(
        &self,
        account_data: &[u8],
    ) -> Option<Arc<ToolboxIdlAccount>> {
        for account in self.accounts.values() {
            if !account_data.starts_with(&account.discriminator) {
                continue;
            }
            if let Some(account_space) = account.space {
                if account_data.len() != account_space {
                    continue;
                }
            }
            return Some(account.clone());
        }
        None
    }

    pub fn guess_instruction(
        &self,
        instruction_data: &[u8],
    ) -> Option<Arc<ToolboxIdlInstruction>> {
        for instruction in self.instructions.values() {
            if !instruction_data.starts_with(&instruction.discriminator) {
                continue;
            }
            return Some(instruction.clone());
        }
        None
    }

    pub fn guess_error(&self, error_code: u64) -> Option<Arc<ToolboxIdlError>> {
        for error in self.errors.values() {
            if error_code != error.code {
                continue;
            }
            return Some(error.clone());
        }
        None
    }
}
