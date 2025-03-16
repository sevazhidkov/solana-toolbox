use std::collections::HashMap;

use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_instruction::ToolboxIdlInstruction;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;

impl ToolboxIdlInstruction {
    pub fn compile_addresses(
        &self,
        instruction_addresses: &HashMap<String, Pubkey>,
    ) -> Result<Vec<AccountMeta>, ToolboxIdlError> {
        let mut instruction_metas = vec![];
        for instruction_account in &self.accounts {
            let instruction_address = *idl_map_get_key_or_else(
                instruction_addresses,
                &instruction_account.name,
                &ToolboxIdlBreadcrumbs::default()
                    .as_val("instruction_addresses"),
            )?;
            if instruction_account.is_writable {
                instruction_metas.push(AccountMeta::new(
                    instruction_address,
                    instruction_account.is_signer,
                ));
            } else {
                instruction_metas.push(AccountMeta::new_readonly(
                    instruction_address,
                    instruction_account.is_signer,
                ));
            }
        }
        Ok(instruction_metas)
    }

    pub fn decompile_addresses(
        &self,
        instruction_metas: &[AccountMeta],
    ) -> Result<HashMap<String, Pubkey>, ToolboxIdlError> {
        if self.accounts.len() != instruction_metas.len() {
            return idl_err(
                "Invalid instruction accounts length",
                &ToolboxIdlBreadcrumbs::default().as_val("instruction_metas"),
            );
        }
        let mut instruction_addresses = HashMap::new();
        for (instruction_account, instruction_meta) in
            self.accounts.iter().zip(instruction_metas.iter())
        {
            instruction_addresses.insert(
                instruction_account.name.to_string(),
                instruction_meta.pubkey,
            );
        }
        Ok(instruction_addresses)
    }
}
