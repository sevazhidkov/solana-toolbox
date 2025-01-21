use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_scoped_object_array_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;

impl ToolboxIdl {
    pub fn compile_instruction_data(
        &self,
        instruction_name: &str,
        instruction_args: &Map<String, Value>,
    ) -> Result<Vec<u8>, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let discriminator = idl_map_get_key_or_else(
            &self.instructions_discriminators,
            instruction_name,
            &breadcrumbs.as_idl("instructions_discriminators"),
        )?;
        let mut instruction_data = vec![];
        instruction_data.extend_from_slice(discriminator);
        for (idl_instruction_arg_object, breadcrumbs) in
            idl_object_get_key_as_scoped_object_array_or_else(
                &self.instructions_args,
                instruction_name,
                &breadcrumbs.with_idl("instruction_args"),
            )?
        {
            let idl_instruction_arg_name = idl_object_get_key_as_str_or_else(
                idl_instruction_arg_object,
                "name",
                &breadcrumbs.as_idl("@"),
            )?;
            let idl_instruction_arg_type = idl_object_get_key_or_else(
                idl_instruction_arg_object,
                "type",
                &breadcrumbs.as_idl(idl_instruction_arg_name),
            )?;
            let instruction_arg = idl_object_get_key_or_else(
                instruction_args,
                idl_instruction_arg_name,
                &breadcrumbs.as_val("&"),
            )?;
            self.type_serialize(
                idl_instruction_arg_type,
                instruction_arg,
                &mut instruction_data,
                &breadcrumbs.with_val(idl_instruction_arg_name),
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
        let discriminator = idl_map_get_key_or_else(
            &self.instructions_discriminators,
            instruction_name,
            &breadcrumbs.as_idl("instructions_discriminators"),
        )?;
        if !instruction_data.starts_with(discriminator) {
            return Err(ToolboxIdlError::InvalidDiscriminator {
                expected: discriminator.to_vec(),
                found: instruction_data.to_vec(),
            });
        }
        let mut data_offset = discriminator.len();
        let mut instruction_args = Map::new();
        for (idl_instruction_arg_object, breadcrumbs) in
            idl_object_get_key_as_scoped_object_array_or_else(
                &self.instructions_args,
                instruction_name,
                &breadcrumbs.with_idl("instruction_args"),
            )?
        {
            let idl_instruction_arg_name = idl_object_get_key_as_str_or_else(
                idl_instruction_arg_object,
                "name",
                &breadcrumbs.as_idl("@"),
            )?;
            let idl_instruction_arg_type = idl_object_get_key_or_else(
                idl_instruction_arg_object,
                "type",
                &breadcrumbs.as_idl(idl_instruction_arg_name),
            )?;
            let (data_arg_size, data_arg_value) = self.type_deserialize(
                idl_instruction_arg_type,
                instruction_data,
                data_offset,
                &breadcrumbs.with_val(idl_instruction_arg_name),
            )?;
            data_offset += data_arg_size;
            instruction_args
                .insert(idl_instruction_arg_name.into(), data_arg_value);
        }
        Ok(instruction_args)
    }

    pub fn guess_instruction_name(
        &self,
        instruction_data: &[u8],
    ) -> Option<&str> {
        for (instruction_name, instruction_discriminator) in
            &self.instructions_discriminators
        {
            if instruction_data.starts_with(instruction_discriminator) {
                return Some(instruction_name);
            }
        }
        None
    }
}
