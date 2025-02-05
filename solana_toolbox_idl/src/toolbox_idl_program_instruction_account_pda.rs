use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_instruction_account_blob::ToolboxIdlProgramInstructionAccountBlob;
use crate::toolbox_idl_utils::idl_iter_get_scoped_values;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramInstructionAccountPda {
    pub seeds: Vec<ToolboxIdlProgramInstructionAccountBlob>,
    pub program: Option<ToolboxIdlProgramInstructionAccountBlob>,
}

impl ToolboxIdlProgramInstructionAccountPda {
    pub(crate) fn try_parse(
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
                    &breadcrumbs,
                )?);
        }
        Ok(Some(ToolboxIdlProgramInstructionAccountPda {
            seeds: pda_seeds,
            program: pda_program,
        }))
    }
}
