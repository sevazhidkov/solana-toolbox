use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_array_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;
use crate::toolbox_idl_utils::idl_u64_from_bytes_at;
use crate::ToolboxIdlBreadcrumbs;

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
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let idl_instruction_args = idl_object_get_key_as_array_or_else(
            &self.instructions_args,
            instruction_name,
            breadcrumbs.context("instruction_args"),
        )?;
        for index in 0..idl_instruction_args.len() {
            let idl_instruction_arg = idl_instruction_args.get(index).unwrap();
            let breadcrumbs = &breadcrumbs.node(&format!("arg[{}]", index));
            let idl_instruction_arg_object = idl_as_object_or_else(
                idl_instruction_arg,
                breadcrumbs.leaf("description"),
            )?;
            let idl_instruction_arg_name = idl_object_get_key_as_str_or_else(
                idl_instruction_arg_object,
                "name",
                breadcrumbs.leaf("description"),
            )?;
            let idl_instruction_arg_type = idl_object_get_key_or_else(
                idl_instruction_arg_object,
                "type",
                breadcrumbs,
            )?;
            let instruction_arg = idl_object_get_key_or_else(
                instruction_args,
                idl_instruction_arg_name,
                breadcrumbs,
            )?;
            self.type_serialize(
                idl_instruction_arg_type,
                instruction_arg,
                &mut instruction_data,
                breadcrumbs,
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
        let idl_instruction_args = idl_object_get_key_as_array_or_else(
            &self.instructions_args,
            instruction_name,
            breadcrumbs,
        )?;
        let data_discriminator =
            idl_u64_from_bytes_at(instruction_data, 0, breadcrumbs)?;
        let expected_discriminator =
            ToolboxIdl::compute_instruction_discriminator(instruction_name);
        if data_discriminator != expected_discriminator {
            return Err(ToolboxIdlError::InvalidDiscriminator {
                found: data_discriminator,
                expected: expected_discriminator,
            });
        }
        let mut instruction_args = Map::new();
        let mut data_offset = size_of_val(&data_discriminator);
        for idl_instruction_arg in idl_instruction_args {
            let idl_instruction_arg_object =
                idl_as_object_or_else(idl_instruction_arg, breadcrumbs)?;
            let idl_instruction_arg_name = idl_object_get_key_as_str_or_else(
                idl_instruction_arg_object,
                "name",
                breadcrumbs,
            )?;
            let idl_instruction_arg_type = idl_object_get_key_or_else(
                idl_instruction_arg_object,
                "type",
                breadcrumbs,
            )?;
            let (data_arg_size, data_arg_value) = self.type_deserialize(
                idl_instruction_arg_type,
                instruction_data,
                data_offset,
                breadcrumbs,
            )?;
            data_offset += data_arg_size;
            instruction_args
                .insert(idl_instruction_arg_name.into(), data_arg_value);
        }
        Ok(instruction_args)
    }
}
