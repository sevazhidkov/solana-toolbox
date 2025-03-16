use std::collections::HashMap;
use std::sync::Arc;

use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_account::ToolboxIdlAccount;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_instruction::ToolboxIdlInstruction;
use crate::toolbox_idl_transaction_error::ToolboxIdlTransactionError;
use crate::toolbox_idl_typedef::ToolboxIdlTypedef;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgram {
    pub typedefs: HashMap<String, Arc<ToolboxIdlTypedef>>,
    pub instructions: HashMap<String, Arc<ToolboxIdlInstruction>>,
    pub accounts: HashMap<String, Arc<ToolboxIdlAccount>>,
    pub errors: HashMap<String, Arc<ToolboxIdlTransactionError>>,
}

impl ToolboxIdlProgram {
    pub fn find_anchor_pda(
        program_id: &Pubkey,
    ) -> Result<Pubkey, ToolboxIdlError> {
        let base = Pubkey::find_program_address(&[], program_id).0;
        Pubkey::create_with_seed(&base, "anchor:idl", program_id)
            .map_err(ToolboxIdlError::Pubkey)
    }

    pub fn guess_idl_instruction(
        &self,
        instruction_data: &[u8],
    ) -> Option<Arc<ToolboxIdlInstruction>> {
        for instruction in self.instructions.values() {
            if instruction_data.starts_with(&instruction.discriminator) {
                return Some(instruction.clone());
            }
        }
        None
    }

    pub fn guess_idl_account(
        &self,
        account_data: &[u8],
    ) -> Option<Arc<ToolboxIdlAccount>> {
        for account in self.accounts.values() {
            if account_data.starts_with(&account.discriminator) {
                return Some(account.clone());
            }
        }
        None
    }

    pub fn guess_idl_error(
        &self,
        error_code: u64,
    ) -> Option<Arc<ToolboxIdlTransactionError>> {
        for error in self.errors.values() {
            if error_code == error.code {
                return Some(error.clone());
            }
        }
        None
    }
}
