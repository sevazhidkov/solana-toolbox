use std::collections::HashMap;

use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey};

use crate::{
    toolbox_idl_utils::{idl_err, idl_map_get_key_or_else},
    ToolboxIdlBreadcrumbs, ToolboxIdlError, ToolboxIdlProgramInstruction,
};

impl ToolboxIdlProgramInstruction {
    pub fn compile_accounts_addresses(
        &self,
        instruction_accounts_addresses: &HashMap<String, Pubkey>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Vec<AccountMeta>, ToolboxIdlError> {
        let mut accounts_metas = vec![];
        for account in &self.accounts {
            let instruction_account_address = *idl_map_get_key_or_else(
                instruction_accounts_addresses,
                &account.name,
                &breadcrumbs.val(),
            )?;
            if account.is_writable {
                accounts_metas.push(AccountMeta::new(
                    instruction_account_address,
                    account.is_signer,
                ));
            } else {
                accounts_metas.push(AccountMeta::new_readonly(
                    instruction_account_address,
                    account.is_signer,
                ));
            }
        }
        Ok(accounts_metas)
    }

    pub fn decompile_accounts_addresses(
        &self,
        instruction_accounts_metas: &[AccountMeta],
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<HashMap<String, Pubkey>, ToolboxIdlError> {
        if self.accounts.len() != instruction_accounts_metas.len() {
            return idl_err(
                "Invalid instruction accounts length",
                &breadcrumbs.val(),
            );
        }
        let mut accounts_addresses = HashMap::new();
        for (account, instruction_account_meta) in
            self.accounts.iter().zip(instruction_accounts_metas.iter())
        {
            accounts_addresses.insert(
                account.name.to_string(),
                instruction_account_meta.pubkey,
            );
        }
        Ok(accounts_addresses)
    }
}
