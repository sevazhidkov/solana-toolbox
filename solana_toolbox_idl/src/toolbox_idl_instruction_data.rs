use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;

impl ToolboxIdl {
    pub fn compile_instruction_data(
        &self,
        instruction_name: &str,
        instruction_args: &Map<String, Value>,
    ) -> Result<Vec<u8>, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let program_instruction = idl_map_get_key_or_else(
            &self.program_instructions,
            instruction_name,
            &breadcrumbs.as_idl("$program_instructions"),
        )?;
        let mut instruction_data = vec![];
        instruction_data.extend_from_slice(&program_instruction.discriminator);
        for (program_instruction_arg_name, program_instruction_arg_def) in
            &program_instruction.args
        {
            let breadcrumbs =
                &breadcrumbs.with_idl(program_instruction_arg_name);
            let instruction_arg = idl_object_get_key_or_else(
                instruction_args,
                program_instruction_arg_name,
                &breadcrumbs.val(),
            )?;
            program_instruction_arg_def.try_serialize(
                self,
                instruction_arg,
                &mut instruction_data,
                &breadcrumbs.with_val(program_instruction_arg_name),
            )?;
        }
        Ok(instruction_data)
    }

    pub fn decompile_instruction_data(
        &self,
        instruction_name: &str,
        instruction_data: &[u8],
    ) -> Result<Map<String, Value>, ToolboxIdlError> {
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
        let mut data_offset = program_instruction.discriminator.len();
        let mut instruction_args = Map::new();
        for (program_instruction_arg_name, program_instruction_arg_def) in
            &program_instruction.args
        {
            let breadcrumbs =
                &breadcrumbs.with_idl(program_instruction_arg_name);
            let (data_arg_size, data_arg_value) =
                program_instruction_arg_def.try_deserialize(
                    self,
                    instruction_data,
                    data_offset,
                    &breadcrumbs.with_val(program_instruction_arg_name),
                )?;
            data_offset += data_arg_size;
            instruction_args
                .insert(program_instruction_arg_name.into(), data_arg_value);
        }
        Ok(instruction_args)
    }
}
