use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_array_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;

impl ToolboxIdl {
    pub fn generate_instruction_data(
        &self,
        instruction_name: &str,
        instruction_args: &Map<String, Value>,
    ) -> Result<Vec<u8>, ToolboxIdlError> {
        let mut data: Vec<u8> = vec![];
        data.extend_from_slice(
            &ToolboxIdl::compute_instruction_discriminator(instruction_name)
                .to_le_bytes(),
        );
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
            self.type_serialize(
                idl_instruction_arg_type,
                instruction_arg,
                &mut data,
            )?;
        }
        Ok(data)
    }
}
