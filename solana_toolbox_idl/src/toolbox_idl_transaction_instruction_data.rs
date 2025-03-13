use serde_json::Value;

use crate::toolbox_idl_program_root::ToolboxIdlProgramRoot;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::ToolboxIdlProgramInstruction;

impl ToolboxIdlProgramRoot {
    pub fn compile_transaction_instruction_data(
        program_instruction: &ToolboxIdlProgramInstruction,
        transaction_instruction_args: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Vec<u8>, ToolboxIdlError> {
        let mut native_instruction_data = vec![];
        native_instruction_data
            .extend_from_slice(&program_instruction.discriminator);
        program_instruction.args_type_full_fields.try_serialize(
            transaction_instruction_args,
            &mut native_instruction_data,
            true,
            &breadcrumbs.with_val("args"),
        )?;
        Ok(native_instruction_data)
    }

    pub fn decompile_transaction_instruction_data(
        program_instruction: &ToolboxIdlProgramInstruction,
        native_instruction_data: &[u8],
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Value, ToolboxIdlError> {
        if !native_instruction_data
            .starts_with(&program_instruction.discriminator)
        {
            return Err(ToolboxIdlError::InvalidDiscriminator {
                expected: program_instruction.discriminator.to_vec(),
                found: native_instruction_data.to_vec(),
            });
        }
        let (_, transaction_instruction_args) =
            program_instruction.args_type_full_fields.try_deserialize(
                native_instruction_data,
                program_instruction.discriminator.len(),
                &breadcrumbs.with_val("args"),
            )?;
        Ok(transaction_instruction_args)
    }
}
