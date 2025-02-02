use serde_json::Value;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;

impl ToolboxIdl {
    pub fn compile_instruction_data(
        &self,
        instruction_name: &str,
        instruction_args: &Value,
    ) -> Result<Vec<u8>, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let program_instruction = idl_map_get_key_or_else(
            &self.program_instructions,
            instruction_name,
            &breadcrumbs.as_idl("$program_instructions"),
        )?;
        let mut instruction_data = vec![];
        instruction_data.extend_from_slice(&program_instruction.discriminator);
        program_instruction.data_type_full.try_serialize(
            instruction_args,
            &mut instruction_data,
            &breadcrumbs.with_val("args"),
        )?;
        Ok(instruction_data)
    }

    pub fn decompile_instruction_data(
        &self,
        instruction_name: &str,
        instruction_data: &[u8],
    ) -> Result<Value, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let program_instruction = idl_map_get_key_or_else(
            &self.program_instructions,
            instruction_name,
            &breadcrumbs.as_idl("$program_instructions"),
        )?;
        if !instruction_data.starts_with(&program_instruction.discriminator) {
            return Err(ToolboxIdlError::InvalidDiscriminator {
                expected: program_instruction.discriminator.to_vec(),
                found: instruction_data.to_vec(),
            });
        }
        let (_, instruction_args) =
            program_instruction.data_type_full.try_deserialize(
                instruction_data,
                program_instruction.discriminator.len(),
                &breadcrumbs.with_val("args"),
            )?;
        Ok(instruction_args)
    }
}
