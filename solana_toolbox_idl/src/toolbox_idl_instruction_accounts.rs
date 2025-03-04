use std::collections::HashMap;
use std::vec;

use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;

impl ToolboxIdl {
    pub fn compile_instruction_accounts(
        &self,
        instruction_name: &str,
        instruction_accounts_addresses: &HashMap<String, Pubkey>,
    ) -> Result<Vec<AccountMeta>, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let mut account_metas = vec![];
        let program_instruction = idl_map_get_key_or_else(
            &self.program_instructions,
            instruction_name,
            &breadcrumbs.as_idl("$program_instructions"),
        )?;
        for program_instruction_account in &program_instruction.accounts {
            let instruction_account_address = *idl_map_get_key_or_else(
                instruction_accounts_addresses,
                &program_instruction_account.name,
                &breadcrumbs.as_val("instruction_accounts_addresses"),
            )?;
            if program_instruction_account.is_writable {
                account_metas.push(AccountMeta::new(
                    instruction_account_address,
                    program_instruction_account.is_signer,
                ));
            } else {
                account_metas.push(AccountMeta::new_readonly(
                    instruction_account_address,
                    program_instruction_account.is_signer,
                ));
            }
        }
        Ok(account_metas)
    }

    pub fn decompile_instruction_accounts(
        &self,
        instruction_name: &str,
        instruction_accounts: &[AccountMeta],
    ) -> Result<HashMap<String, Pubkey>, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let program_instruction = idl_map_get_key_or_else(
            &self.program_instructions,
            instruction_name,
            &breadcrumbs.as_idl("$program_instructions"),
        )?;
        if program_instruction.accounts.len() != instruction_accounts.len() {
            return idl_err(
                "Invalid instruction accounts length",
                &breadcrumbs.val(),
            );
        }
        let mut instruction_accounts_addresses = HashMap::new();
        for (program_instruction_account, instruction_account_meta) in
            program_instruction
                .accounts
                .iter()
                .zip(instruction_accounts.iter())
        {
            instruction_accounts_addresses.insert(
                program_instruction_account.name.to_string(),
                instruction_account_meta.pubkey,
            );
        }
        Ok(instruction_accounts_addresses)
    }
}
