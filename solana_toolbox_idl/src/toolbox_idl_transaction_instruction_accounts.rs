use std::collections::HashMap;
use std::vec;

use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;
use crate::ToolboxIdlProgramInstruction;

impl ToolboxIdl {
    pub fn compile_transaction_instruction_accounts(
        program_instruction: &ToolboxIdlProgramInstruction,
        transaction_instruction_accounts_addresses: &HashMap<String, Pubkey>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Vec<AccountMeta>, ToolboxIdlError> {
        let mut native_instruction_account_metas = vec![];
        for program_instruction_account in &program_instruction.accounts {
            let transaction_instruction_account_address =
                *idl_map_get_key_or_else(
                    transaction_instruction_accounts_addresses,
                    &program_instruction_account.name,
                    &breadcrumbs.as_val("instruction_accounts_addresses"),
                )?;
            if program_instruction_account.is_writable {
                native_instruction_account_metas.push(AccountMeta::new(
                    transaction_instruction_account_address,
                    program_instruction_account.is_signer,
                ));
            } else {
                native_instruction_account_metas.push(
                    AccountMeta::new_readonly(
                        transaction_instruction_account_address,
                        program_instruction_account.is_signer,
                    ),
                );
            }
        }
        Ok(native_instruction_account_metas)
    }

    // TODO - should this be on the program_instruction API directly ?
    pub fn decompile_transaction_instruction_accounts(
        program_instruction: &ToolboxIdlProgramInstruction,
        native_instruction_accounts: &[AccountMeta],
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<HashMap<String, Pubkey>, ToolboxIdlError> {
        if program_instruction.accounts.len()
            != native_instruction_accounts.len()
        {
            return idl_err(
                "Invalid instruction accounts length",
                &breadcrumbs.val(),
            );
        }
        let mut transaction_instruction_accounts_addresses = HashMap::new();
        for (program_instruction_account, native_instruction_account_meta) in
            program_instruction
                .accounts
                .iter()
                .zip(native_instruction_accounts.iter())
        {
            transaction_instruction_accounts_addresses.insert(
                program_instruction_account.name.to_string(),
                native_instruction_account_meta.pubkey,
            );
        }
        Ok(transaction_instruction_accounts_addresses)
    }
}
