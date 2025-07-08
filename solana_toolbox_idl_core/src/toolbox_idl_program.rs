use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_account::ToolboxIdlAccount;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_event::ToolboxIdlEvent;
use crate::toolbox_idl_instruction::ToolboxIdlInstruction;
use crate::toolbox_idl_typedef::ToolboxIdlTypedef;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ToolboxIdlProgramMetadata {
    pub address: Option<Pubkey>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub docs: Option<Value>,
    pub version: Option<String>,
    pub spec: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ToolboxIdlProgram {
    pub metadata: ToolboxIdlProgramMetadata,
    pub typedefs: HashMap<String, Arc<ToolboxIdlTypedef>>,
    pub accounts: HashMap<String, Arc<ToolboxIdlAccount>>,
    pub instructions: HashMap<String, Arc<ToolboxIdlInstruction>>,
    pub events: HashMap<String, Arc<ToolboxIdlEvent>>,
    pub errors: HashMap<String, Arc<ToolboxIdlError>>,
}

impl ToolboxIdlProgram {
    pub fn guess_account(
        &self,
        account_data: &[u8],
    ) -> Option<Arc<ToolboxIdlAccount>> {
        for account in self.accounts.values() {
            if account.check(account_data).is_ok() {
                return Some(account.clone());
            }
        }
        None
    }

    pub fn guess_instruction(
        &self,
        instruction_data: &[u8],
    ) -> Option<Arc<ToolboxIdlInstruction>> {
        for instruction in self.instructions.values() {
            if instruction.check_payload(instruction_data).is_ok() {
                return Some(instruction.clone());
            }
        }
        None
    }

    pub fn guess_event(
        &self,
        event_data: &[u8],
    ) -> Option<Arc<ToolboxIdlEvent>> {
        for event in self.events.values() {
            if event.check(event_data).is_ok() {
                return Some(event.clone());
            }
        }
        None
    }

    pub fn guess_error(&self, error_code: u64) -> Option<Arc<ToolboxIdlError>> {
        for error in self.errors.values() {
            if error_code == error.code {
                return Some(error.clone());
            }
        }
        None
    }
}
