use std::collections::HashMap;

use serde_json::Map;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_account::ToolboxIdlAccount;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_instruction_account_blob::ToolboxIdlProgramInstructionAccountBlob;
use crate::toolbox_idl_transaction_instruction::ToolboxIdlTransactionInstruction;
use crate::toolbox_idl_utils::idl_iter_get_scoped_values;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::ToolboxIdlProgramAccount;
use crate::ToolboxIdlProgramInstruction;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramInstructionAccountPda {
    pub seeds: Vec<ToolboxIdlProgramInstructionAccountBlob>,
    pub program: Option<ToolboxIdlProgramInstructionAccountBlob>,
}

impl ToolboxIdlProgramInstructionAccountPda {
    pub fn try_parse(
        idl_instruction_account_pda: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Option<ToolboxIdlProgramInstructionAccountPda>, ToolboxIdlError>
    {
        let mut pda_seeds = vec![];
        if let Some(idl_instruction_account_pda_seeds) =
            idl_object_get_key_as_array(idl_instruction_account_pda, "seeds")
        {
            for (_, idl_instruction_account_pda_seed, breadcrumbs) in
                idl_iter_get_scoped_values(
                    idl_instruction_account_pda_seeds,
                    breadcrumbs,
                )?
            {
                pda_seeds.push(
                    ToolboxIdlProgramInstructionAccountBlob::try_parse(
                        idl_instruction_account_pda_seed,
                        &breadcrumbs,
                    )?,
                );
            }
        }
        let mut pda_program = None;
        if let Some(idl_instruction_account_pda_program) =
            idl_instruction_account_pda.get("program")
        {
            pda_program =
                Some(ToolboxIdlProgramInstructionAccountBlob::try_parse(
                    idl_instruction_account_pda_program,
                    breadcrumbs,
                )?);
        }
        Ok(Some(ToolboxIdlProgramInstructionAccountPda {
            seeds: pda_seeds,
            program: pda_program,
        }))
    }

    pub fn try_resolve(
        &self,
        program_instruction: &ToolboxIdlProgramInstruction,
        program_accounts: &HashMap<String, ToolboxIdlProgramAccount>,
        transaction_instruction: &ToolboxIdlTransactionInstruction,
        transaction_instruction_accounts: &HashMap<String, ToolboxIdlAccount>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Pubkey, ToolboxIdlError> {
        let mut pda_seeds_bytes = vec![];
        for pda_seed_blob in &self.seeds {
            pda_seeds_bytes.push(pda_seed_blob.try_resolve(
                program_instruction,
                program_accounts,
                transaction_instruction,
                transaction_instruction_accounts,
                breadcrumbs,
            )?);
        }
        let pda_program_id = if let Some(pda_program_blob) = &self.program {
            let pda_program_id_bytes = pda_program_blob.try_resolve(
                program_instruction,
                program_accounts,
                transaction_instruction,
                transaction_instruction_accounts,
                &breadcrumbs.with_idl("program"),
            )?;
            Pubkey::new_from_array(pda_program_id_bytes.try_into().map_err(
                |err| {
                    ToolboxIdlError::Custom {
                        failure: format!("value:{:?}", err), // TODO - better error handling and breadcrumbs
                        context: breadcrumbs.as_idl("program_id"),
                    }
                },
            )?)
        } else {
            transaction_instruction.program_id
        };
        let mut pda_seeds_slices = vec![];
        for pda_seed_bytes in pda_seeds_bytes.iter() {
            pda_seeds_slices.push(&pda_seed_bytes[..]);
        }
        Ok(Pubkey::find_program_address(&pda_seeds_slices, &pda_program_id).0)
    }
}
