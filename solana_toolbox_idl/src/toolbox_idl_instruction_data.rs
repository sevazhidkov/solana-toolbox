use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_object_get_key_as_array_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;
use crate::toolbox_idl_utils::idl_u64_from_bytes_at;

impl ToolboxIdl {
    pub fn compile_instruction_data(
        &self,
        instruction_name: &str,
        instruction_args: &Map<String, Value>,
    ) -> Result<Vec<u8>, ToolboxIdlError> {
        let mut instruction_data = vec![];
        instruction_data.extend_from_slice(bytemuck::bytes_of(
            &ToolboxIdl::compute_instruction_discriminator(instruction_name),
        ));
        let idl_instruction_args = idl_object_get_key_as_array_or_else(
            &self.instructions_args,
            instruction_name,
            "instructions args",
        )?;
        for idl_instruction_arg in idl_instruction_args {
            let idl_instruction_arg_object =
                idl_as_object_or_else(idl_instruction_arg, "instruction arg")?;
            let idl_instruction_arg_name = idl_object_get_key_as_str_or_else(
                idl_instruction_arg_object,
                "name",
                "instruction arg",
            )?;
            let idl_instruction_arg_type = idl_object_get_key_or_else(
                idl_instruction_arg_object,
                "type",
                "instruction arg",
            )?;
            let instruction_arg = idl_object_get_key_or_else(
                instruction_args,
                idl_instruction_arg_name,
                "instruction params",
            )?;
            self.type_writer(
                idl_instruction_arg_type,
                instruction_arg,
                &mut instruction_data,
            )?;
        }
        Ok(instruction_data)
    }

    pub fn decompile_instruction_data(
        &self,
        instruction_name: &str,
        instruction_data: &[u8],
    ) -> Result<Map<String, Value>, ToolboxIdlError> {
        let idl_instruction_args = idl_object_get_key_as_array_or_else(
            &self.instructions_args,
            instruction_name,
            "instructions args",
        )?;
        let data_discriminator = idl_u64_from_bytes_at(instruction_data, 0)?;
        let expected_discriminator =
            ToolboxIdl::compute_instruction_discriminator(instruction_name);
        if data_discriminator != expected_discriminator {
            return idl_err(&format!(
                "invalid discriminator: found {:016X}, expected {:016X}",
                data_discriminator, expected_discriminator
            ));
        }
        let mut instruction_args = Map::new();
        let mut data_offset = size_of_val(&data_discriminator);
        for idl_instruction_arg in idl_instruction_args {
            let idl_instruction_arg_object =
                idl_as_object_or_else(idl_instruction_arg, "instruction arg")?;
            let idl_instruction_arg_name = idl_object_get_key_as_str_or_else(
                idl_instruction_arg_object,
                "name",
                "instruction arg",
            )?;
            let idl_instruction_arg_type = idl_object_get_key_or_else(
                idl_instruction_arg_object,
                "type",
                "instruction arg",
            )?;
            let (data_arg_size, data_arg_value) = self.type_reader(
                idl_instruction_arg_type,
                instruction_data,
                data_offset,
            )?;
            data_offset += data_arg_size;
            instruction_args
                .insert(idl_instruction_arg_name.into(), data_arg_value);
        }
        Ok(instruction_args)
    }
}
