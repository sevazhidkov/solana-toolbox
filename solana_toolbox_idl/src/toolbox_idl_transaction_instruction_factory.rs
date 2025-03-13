use std::collections::HashMap;

use convert_case::Case;
use convert_case::Casing;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_idl_account::ToolboxIdlAccount;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_root::ToolboxIdlProgramRoot;
use crate::toolbox_idl_transaction_instruction::ToolboxIdlTransactionInstruction;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;

impl ToolboxIdlProgramRoot {
    // pub async fn resolve_instruction(
    // &self,
    // endpoint: &mut ToolboxEndpoint,
    // instruction: &ToolboxIdlTransactionInstruction,
    // ) -> Result<Instruction, ToolboxIdlError> {
    // let instruction_accounts_addresses = self
    // .resolve_instruction_accounts_addresses(endpoint, instruction)
    // .await?;
    // let instruction = ToolboxIdlTransactionInstruction {
    // program_id: instruction.program_id,
    // name: instruction.name.clone(),
    // accounts_addresses: instruction_accounts_addresses,
    // args: instruction.args.clone(),
    // };
    // Ok(Instruction {
    // program_id: instruction.program_id,
    // accounts: self.compile_transaction_instruction_accounts(
    // &instruction.name,
    // &instruction.accounts_addresses,
    // )?,
    // data: self.compile_transaction_instruction_data(
    // &instruction.name,
    // &instruction.args,
    // )?,
    // })
    // }

    pub async fn try_resolve(
        &self,
        endpoint: &mut ToolboxEndpoint,
    ) -> Result<ToolboxIdlTransactionInstruction, ToolboxIdlError> {
        let mut transaction_instruction_accounts_addresses =
            transaction_instruction.accounts_addresses.clone();
        let mut instruction_accounts = self
            .get_accounts_by_name(
                endpoint,
                &transaction_instruction_accounts_addresses,
            )
            .await?;
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let program_instruction = idl_map_get_key_or_else(
            &self.instructions,
            &transaction_instruction.name,
            &breadcrumbs.as_idl("$program_instructions"),
        )?;
        loop {
            let mut made_progress = false;
            for program_instruction_account in &program_instruction.accounts {
                if transaction_instruction_accounts_addresses
                    .contains_key(&program_instruction_account.name)
                {
                    continue;
                }
                if let Ok(instruction_account_address) = self
                    .find_instruction_account_address(
                        transaction_instruction,
                        &instruction_accounts,
                        &program_instruction_account.name,
                        breadcrumbs,
                    )
                {
                    made_progress = true;
                    transaction_instruction_accounts_addresses.insert(
                        program_instruction_account.name.to_string(),
                        instruction_account_address,
                    );
                    if let Ok(Some(instruction_account)) = self
                        .get_account(endpoint, &instruction_account_address)
                        .await
                    {
                        instruction_accounts.insert(
                            program_instruction_account.name.to_string(),
                            instruction_account,
                        );
                    }
                }
            }
            if !made_progress {
                break;
            }
        }
        Ok((
            transaction_instruction_accounts_addresses,
            instruction_accounts,
        ))
    }

    pub fn find_instruction_accounts_addresses(
        &self,
        instruction: &ToolboxIdlTransactionInstruction,
        instruction_accounts: &HashMap<String, ToolboxIdlAccount>,
    ) -> Result<HashMap<String, Pubkey>, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let program_instruction = idl_map_get_key_or_else(
            &self.instructions,
            &instruction.name,
            &breadcrumbs.as_idl("$program_instructions"),
        )?;
        let mut instruction_accounts_addresses =
            instruction.accounts_addresses.clone();
        for program_instruction_account in &program_instruction.accounts {
            if !instruction_accounts_addresses
                .contains_key(&program_instruction_account.name)
            {
                instruction_accounts_addresses.insert(
                    program_instruction_account.name.to_string(),
                    self.find_instruction_account_address(
                        instruction,
                        instruction_accounts,
                        &program_instruction_account.name,
                        breadcrumbs,
                    )?,
                );
            }
        }
        Ok(instruction_accounts_addresses)
    }

    pub fn find_instruction_account_address(
        &self,
        instruction: &ToolboxIdlTransactionInstruction,
        instruction_accounts: &HashMap<String, ToolboxIdlAccount>,
        instruction_account_name: &str,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Pubkey, ToolboxIdlError> {
        let program_instruction = idl_map_get_key_or_else(
            &self.instructions,
            &instruction.name,
            &breadcrumbs.as_idl("$program_instructions"),
        )?;
        for program_instruction_account in &program_instruction.accounts {
            if program_instruction_account.name.to_case(Case::Snake)
                == instruction_account_name.to_case(Case::Snake)
            {
                return program_instruction_account.try_resolve(
                    self,
                    instruction,
                    instruction_accounts,
                    breadcrumbs,
                );
            }
        }
        idl_err(
            "Unknown instruction account name",
            &breadcrumbs.as_val(instruction_account_name),
        )
    }
}
