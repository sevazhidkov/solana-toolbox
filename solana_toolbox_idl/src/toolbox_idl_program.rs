use std::collections::HashMap;

use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_account::ToolboxIdlAccount;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_instruction::ToolboxIdlInstruction;
use crate::toolbox_idl_transaction_error::ToolboxIdlTransactionError;
use crate::toolbox_idl_typedef::ToolboxIdlTypedef;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgram {
    // TODO - should this be not "pub", provide getters instead ?
    pub typedefs: HashMap<String, ToolboxIdlTypedef>,
    pub instructions: HashMap<String, ToolboxIdlInstruction>,
    pub accounts: HashMap<String, ToolboxIdlAccount>,
    pub errors: HashMap<String, ToolboxIdlTransactionError>,
}

impl ToolboxIdlProgram {
    pub fn find_anchor_idl(
        program_id: &Pubkey,
    ) -> Result<Pubkey, ToolboxIdlError> {
        let base = Pubkey::find_program_address(&[], program_id).0;
        Pubkey::create_with_seed(&base, "anchor:idl", program_id)
            .map_err(ToolboxIdlError::Pubkey)
    }

    pub fn guess_instruction(
        &self,
        instruction_data: &[u8],
    ) -> Result<&ToolboxIdlInstruction, ToolboxIdlError> {
        for instruction in self.instructions.values() {
            if instruction_data.starts_with(&instruction.discriminator) {
                return Ok(instruction);
            }
        }
        Err(ToolboxIdlError::CouldNotFindInstruction {})
    }

    pub fn guess_account(
        &self,
        account_data: &[u8],
    ) -> Result<&ToolboxIdlAccount, ToolboxIdlError> {
        for account in self.accounts.values() {
            if account_data.starts_with(&account.discriminator) {
                return Ok(account);
            }
        }
        Err(ToolboxIdlError::CouldNotFindAccount {})
    }
}
