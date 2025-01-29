use std::str::FromStr;

use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_typedef::ToolboxIdlProgramTypedef;
use crate::toolbox_idl_program_typedef_primitive::ToolboxIdlProgramTypedefPrimitiveKind;
use crate::toolbox_idl_utils::idl_as_array_or_else;
use crate::toolbox_idl_utils::idl_as_bool_or_else;
use crate::toolbox_idl_utils::idl_as_f64_or_else;
use crate::toolbox_idl_utils::idl_as_i128_or_else;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_as_str_or_else;
use crate::toolbox_idl_utils::idl_as_u128_or_else;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;

impl ToolboxIdlProgramTypedef {
    pub(crate) fn try_serialize(
        &self,
        idl: &ToolboxIdl,
        value: &Value,
        data: &mut Vec<u8>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        match self {
            ToolboxIdlProgramTypedef::Defined { name } => {
                ToolboxIdlProgramTypedef::try_serialize_defined(
                    idl,
                    name,
                    value,
                    data,
                    &breadcrumbs.with_idl(name),
                )
            },
            ToolboxIdlProgramTypedef::Option { content_typedef } => {
                ToolboxIdlProgramTypedef::try_serialize_option(
                    idl,
                    content_typedef,
                    value,
                    data,
                    &breadcrumbs.with_idl("option"),
                )
            },
            ToolboxIdlProgramTypedef::Vec { items_typedef } => {
                ToolboxIdlProgramTypedef::try_serialize_vec(
                    idl,
                    items_typedef,
                    value,
                    data,
                    &breadcrumbs.with_idl("vec"),
                )
            },
            ToolboxIdlProgramTypedef::Array { length, items_typedef } => {
                ToolboxIdlProgramTypedef::try_serialize_array(
                    idl,
                    *length,
                    items_typedef,
                    value,
                    data,
                    &breadcrumbs.with_idl("array"),
                )
            },
            ToolboxIdlProgramTypedef::Struct { fields } => {
                ToolboxIdlProgramTypedef::try_serialize_struct(
                    idl,
                    fields,
                    value,
                    data,
                    &breadcrumbs.with_idl("struct"),
                )
            },
            ToolboxIdlProgramTypedef::Enum { variants } => {
                ToolboxIdlProgramTypedef::try_serialize_enum(
                    variants,
                    value,
                    data,
                    &breadcrumbs.with_idl("enum"),
                )
            },
            ToolboxIdlProgramTypedef::Primitive { kind } => {
                ToolboxIdlProgramTypedef::try_serialize_primitive(
                    kind,
                    value,
                    data,
                    breadcrumbs,
                )
            },
        }
    }

    fn try_serialize_defined(
        idl: &ToolboxIdl,
        program_typedef_defined_name: &str,
        value: &Value,
        data: &mut Vec<u8>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        let program_typedef = idl_map_get_key_or_else(
            &idl.program_typedefs,
            program_typedef_defined_name,
            &breadcrumbs.as_idl("$program_typedefs"),
        )?;
        program_typedef.try_serialize(idl, value, data, breadcrumbs)
    }

    fn try_serialize_option(
        idl: &ToolboxIdl,
        program_typedef_option_content_typedef: &ToolboxIdlProgramTypedef,
        value: &Value,
        data: &mut Vec<u8>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        if value.is_null() {
            data.push(0);
            Ok(())
        } else {
            data.push(1);
            program_typedef_option_content_typedef.try_serialize(
                idl,
                value,
                data,
                breadcrumbs,
            )
        }
    }

    fn try_serialize_vec(
        idl: &ToolboxIdl,
        program_typedef_vec_items_typedef: &ToolboxIdlProgramTypedef,
        value: &Value,
        data: &mut Vec<u8>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        let value_array =
            idl_as_array_or_else(value, &breadcrumbs.as_val("vec"))?;
        let value_length = u32::try_from(value_array.len()).unwrap();
        data.extend_from_slice(bytemuck::bytes_of::<u32>(&value_length));
        for (index, value_item) in value_array.iter().enumerate() {
            program_typedef_vec_items_typedef.try_serialize(
                idl,
                value_item,
                data,
                &breadcrumbs.with_val(&format!("[{}]", index)),
            )?;
        }
        Ok(())
    }

    fn try_serialize_array(
        idl: &ToolboxIdl,
        program_typedef_array_length: u32,
        program_typedef_array_items_typedef: &ToolboxIdlProgramTypedef,
        value: &Value,
        data: &mut Vec<u8>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        let value_array =
            idl_as_array_or_else(value, &breadcrumbs.as_val("array"))?;
        if value_array.len() != program_typedef_array_length as usize {
            return idl_err(
            &format!(
                "value array is not the correct size: expected {} items, found {} items",
                program_typedef_array_length,
                value_array.len()
            ),
            &breadcrumbs.as_idl("value array"),
        );
        }
        for (index, value_item) in value_array.iter().enumerate() {
            program_typedef_array_items_typedef.try_serialize(
                idl,
                value_item,
                data,
                &breadcrumbs.with_val(&format!("[{}]", index)),
            )?;
        }
        Ok(())
    }

    fn try_serialize_struct(
        idl: &ToolboxIdl,
        program_typedef_struct_fields: &[(String, ToolboxIdlProgramTypedef)],
        value: &Value,
        data: &mut Vec<u8>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        let value_object =
            idl_as_object_or_else(value, &breadcrumbs.as_val("struct"))?;
        for (
            program_typedef_struct_field_name,
            program_typedef_struct_field_typedef,
        ) in program_typedef_struct_fields
        {
            let breadcrumbs =
                &breadcrumbs.with_idl(program_typedef_struct_field_name);
            let value_field = idl_object_get_key_or_else(
                value_object,
                program_typedef_struct_field_name,
                &breadcrumbs.val(),
            )?;
            program_typedef_struct_field_typedef.try_serialize(
                idl,
                value_field,
                data,
                &breadcrumbs.with_val(program_typedef_struct_field_name),
            )?;
        }
        Ok(())
    }

    fn try_serialize_enum(
        program_typedef_enum_variants: &[String],
        value: &Value,
        data: &mut Vec<u8>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        let value_string =
            idl_as_str_or_else(value, &breadcrumbs.as_val("enum"))?;
        for (program_typedef_enum_value, program_typedef_enum_variant) in
            program_typedef_enum_variants.iter().enumerate()
        {
            if program_typedef_enum_variant == value_string {
                data.push(u8::try_from(program_typedef_enum_value).unwrap());
                return Ok(());
            }
        }
        idl_err(
            "could not find matching enum",
            &breadcrumbs.as_val(value_string),
        )
    }

    fn try_serialize_primitive(
        program_typedef_primitive_kind: &ToolboxIdlProgramTypedefPrimitiveKind,
        value: &Value,
        data: &mut Vec<u8>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        let context = &breadcrumbs.val();
        macro_rules! write_data_using_u_number {
            ($type:ident) => {
                let value_integer = idl_as_u128_or_else(value, context)?;
                let value_typed =
                    $type::try_from(value_integer).map_err(|err| {
                        ToolboxIdlError::InvalidInteger {
                            conversion: err,
                            context: context.clone(),
                        }
                    })?;
                data.extend_from_slice(bytemuck::bytes_of::<$type>(
                    &value_typed,
                ));
            };
        }
        macro_rules! write_data_using_i_number {
            ($type:ident) => {
                let value_integer = idl_as_i128_or_else(value, context)?;
                let value_typed =
                    $type::try_from(value_integer).map_err(|err| {
                        ToolboxIdlError::InvalidInteger {
                            conversion: err,
                            context: context.clone(),
                        }
                    })?;
                data.extend_from_slice(bytemuck::bytes_of::<$type>(
                    &value_typed,
                ));
            };
        }
        match program_typedef_primitive_kind {
            ToolboxIdlProgramTypedefPrimitiveKind::U8 => {
                write_data_using_u_number!(u8);
            },
            ToolboxIdlProgramTypedefPrimitiveKind::U16 => {
                write_data_using_u_number!(u16);
            },
            ToolboxIdlProgramTypedefPrimitiveKind::U32 => {
                write_data_using_u_number!(u32);
            },
            ToolboxIdlProgramTypedefPrimitiveKind::U64 => {
                write_data_using_u_number!(u64);
            },
            ToolboxIdlProgramTypedefPrimitiveKind::U128 => {
                let value_integer = idl_as_u128_or_else(value, context)?;
                data.extend_from_slice(bytemuck::bytes_of::<u128>(
                    &value_integer,
                ));
            },
            ToolboxIdlProgramTypedefPrimitiveKind::I8 => {
                write_data_using_i_number!(i8);
            },
            ToolboxIdlProgramTypedefPrimitiveKind::I16 => {
                write_data_using_i_number!(i16);
            },
            ToolboxIdlProgramTypedefPrimitiveKind::I32 => {
                write_data_using_i_number!(i32);
            },
            ToolboxIdlProgramTypedefPrimitiveKind::I64 => {
                write_data_using_i_number!(i64);
            },
            ToolboxIdlProgramTypedefPrimitiveKind::I128 => {
                let value_integer = idl_as_i128_or_else(value, context)?;
                data.extend_from_slice(bytemuck::bytes_of::<i128>(
                    &value_integer,
                ));
            },
            ToolboxIdlProgramTypedefPrimitiveKind::F32 => {
                let value_floating = idl_as_f64_or_else(value, context)? as f32;
                data.extend_from_slice(bytemuck::bytes_of::<f32>(
                    &value_floating,
                ));
            },
            ToolboxIdlProgramTypedefPrimitiveKind::F64 => {
                let value_floating = idl_as_f64_or_else(value, context)?;
                data.extend_from_slice(bytemuck::bytes_of::<f64>(
                    &value_floating,
                ));
            },
            ToolboxIdlProgramTypedefPrimitiveKind::Boolean => {
                data.push(
                    if idl_as_bool_or_else(value, context)? { 1 } else { 0 },
                );
            },
            ToolboxIdlProgramTypedefPrimitiveKind::String => {
                let value_str = idl_as_str_or_else(value, context)?;
                let value_length = u32::try_from(value_str.len()).unwrap();
                data.extend_from_slice(bytemuck::bytes_of::<u32>(
                    &value_length,
                ));
                data.extend_from_slice(value_str.as_bytes());
            },
            ToolboxIdlProgramTypedefPrimitiveKind::PublicKey => {
                let value_str = idl_as_str_or_else(value, context)?;
                let value_pubkey =
                    Pubkey::from_str(value_str).map_err(|err| {
                        ToolboxIdlError::InvalidPubkey {
                            parsing: err,
                            context: context.clone(),
                        }
                    })?;
                data.extend_from_slice(bytemuck::bytes_of::<Pubkey>(
                    &value_pubkey,
                ));
            },
        };
        Ok(())
    }
}
