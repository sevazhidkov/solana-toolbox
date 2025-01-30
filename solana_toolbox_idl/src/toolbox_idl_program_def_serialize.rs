use std::str::FromStr;

use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_def::ToolboxIdlProgramDef;
use crate::toolbox_idl_program_def_primitive::ToolboxIdlProgramDefPrimitive;
use crate::toolbox_idl_utils::idl_as_array_or_else;
use crate::toolbox_idl_utils::idl_as_bool_or_else;
use crate::toolbox_idl_utils::idl_as_bytes_or_else;
use crate::toolbox_idl_utils::idl_as_f64_or_else;
use crate::toolbox_idl_utils::idl_as_i128_or_else;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_as_str_or_else;
use crate::toolbox_idl_utils::idl_as_u128_or_else;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_map_err_invalid_integer;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;
use crate::toolbox_idl_utils::idl_ok_or_else;

impl ToolboxIdlProgramDef {
    pub(crate) fn try_serialize(
        &self,
        idl: &ToolboxIdl,
        value: &Value,
        data: &mut Vec<u8>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        match self {
            ToolboxIdlProgramDef::Defined { name, generics } => {
                ToolboxIdlProgramDef::try_serialize_defined(
                    idl,
                    name,
                    generics,
                    value,
                    data,
                    &breadcrumbs.with_idl(name),
                )
            },
            ToolboxIdlProgramDef::Option { content } => {
                ToolboxIdlProgramDef::try_serialize_option(
                    idl,
                    content,
                    value,
                    data,
                    &breadcrumbs.with_idl("option"),
                )
            },
            ToolboxIdlProgramDef::Vec { items } => {
                ToolboxIdlProgramDef::try_serialize_vec(
                    idl,
                    items,
                    value,
                    data,
                    &breadcrumbs.with_idl("vec"),
                )
            },
            ToolboxIdlProgramDef::Array { length, items } => {
                ToolboxIdlProgramDef::try_serialize_array(
                    idl,
                    length,
                    items,
                    value,
                    data,
                    &breadcrumbs.with_idl("array"),
                )
            },
            ToolboxIdlProgramDef::Struct { fields } => {
                ToolboxIdlProgramDef::try_serialize_struct(
                    idl,
                    fields,
                    value,
                    data,
                    &breadcrumbs.with_idl("struct"),
                )
            },
            ToolboxIdlProgramDef::Enum { variants } => {
                ToolboxIdlProgramDef::try_serialize_enum(
                    variants,
                    value,
                    data,
                    &breadcrumbs.with_idl("enum"),
                )
            },
            ToolboxIdlProgramDef::Primitive { primitive } => {
                ToolboxIdlProgramDef::try_serialize_primitive(
                    primitive,
                    value,
                    data,
                    breadcrumbs,
                )
            },
            ToolboxIdlProgramDef::Generic { symbol } => todo!(),
            ToolboxIdlProgramDef::Const { literal } => todo!(),
        }
    }

    fn try_serialize_defined(
        idl: &ToolboxIdl,
        program_def_defined_name: &str,
        program_def_defined_generics: &[ToolboxIdlProgramDef],
        value: &Value,
        data: &mut Vec<u8>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        let program_type = idl_map_get_key_or_else(
            &idl.program_types,
            program_def_defined_name,
            &breadcrumbs.as_idl("$program_types"),
        )?;
        program_type.def.try_serialize(idl, value, data, breadcrumbs)
    }

    fn try_serialize_option(
        idl: &ToolboxIdl,
        program_def_option_content_def: &ToolboxIdlProgramDef,
        value: &Value,
        data: &mut Vec<u8>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        if value.is_null() {
            data.push(0);
            Ok(())
        } else {
            data.push(1);
            program_def_option_content_def.try_serialize(
                idl,
                value,
                data,
                breadcrumbs,
            )
        }
    }

    fn try_serialize_vec(
        idl: &ToolboxIdl,
        program_def_vec_items_def: &ToolboxIdlProgramDef,
        value: &Value,
        data: &mut Vec<u8>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        let value_array =
            idl_as_array_or_else(value, &breadcrumbs.as_val("vec"))?;
        let value_length = u32::try_from(value_array.len()).unwrap();
        data.extend_from_slice(bytemuck::bytes_of::<u32>(&value_length));
        for (index, value_item) in value_array.iter().enumerate() {
            program_def_vec_items_def.try_serialize(
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
        program_def_array_length_def: &ToolboxIdlProgramDef,
        program_def_array_items_def: &ToolboxIdlProgramDef,
        value: &Value,
        data: &mut Vec<u8>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        let value_array =
            idl_as_array_or_else(value, &breadcrumbs.as_val("array"))?;
        let program_def_array_length = idl_ok_or_else(
            program_def_array_length_def.as_const_literal(),
            "expected a literal",
            &breadcrumbs.as_idl("length"),
        )?;
        if value_array.len() != *program_def_array_length {
            return idl_err(
            &format!(
                "value array is not the correct size: expected {} items, found {} items",
                program_def_array_length,
                value_array.len()
            ),
            &breadcrumbs.as_idl("value array"),
        );
        }
        for (index, value_item) in value_array.iter().enumerate() {
            program_def_array_items_def.try_serialize(
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
        program_def_struct_fields: &[(String, ToolboxIdlProgramDef)],
        value: &Value,
        data: &mut Vec<u8>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        let value_object =
            idl_as_object_or_else(value, &breadcrumbs.as_val("struct"))?;
        for (program_def_struct_field_name, program_def_struct_field_def) in
            program_def_struct_fields
        {
            let breadcrumbs =
                &breadcrumbs.with_idl(program_def_struct_field_name);
            let value_field = idl_object_get_key_or_else(
                value_object,
                program_def_struct_field_name,
                &breadcrumbs.val(),
            )?;
            program_def_struct_field_def.try_serialize(
                idl,
                value_field,
                data,
                &breadcrumbs.with_val(program_def_struct_field_name),
            )?;
        }
        Ok(())
    }

    fn try_serialize_enum(
        program_def_enum_variants: &[(String, Vec<ToolboxIdlProgramDef>)],
        value: &Value,
        data: &mut Vec<u8>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        let value_string =
            idl_as_str_or_else(value, &breadcrumbs.as_val("enum"))?;
        for (program_def_enum_value, program_def_enum_variant) in
            program_def_enum_variants.iter().enumerate()
        {
            if program_def_enum_variant.0 == value_string {
                data.push(u8::try_from(program_def_enum_value).unwrap());
                // TODO - support enum variant fields
                return Ok(());
            }
        }
        idl_err(
            "could not find matching enum",
            &breadcrumbs.as_val(value_string),
        )
    }

    fn try_serialize_primitive(
        program_def_primitive: &ToolboxIdlProgramDefPrimitive,
        value: &Value,
        data: &mut Vec<u8>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        let context = &breadcrumbs.val();
        macro_rules! write_data_using_u_number {
            ($type:ident) => {
                let value_integer = idl_as_u128_or_else(value, context)?;
                let value_typed = idl_map_err_invalid_integer(
                    $type::try_from(value_integer),
                    context,
                )?;
                data.extend_from_slice(bytemuck::bytes_of::<$type>(
                    &value_typed,
                ));
            };
        }
        macro_rules! write_data_using_i_number {
            ($type:ident) => {
                let value_integer = idl_as_i128_or_else(value, context)?;
                let value_typed = idl_map_err_invalid_integer(
                    $type::try_from(value_integer),
                    context,
                )?;
                data.extend_from_slice(bytemuck::bytes_of::<$type>(
                    &value_typed,
                ));
            };
        }
        match program_def_primitive {
            ToolboxIdlProgramDefPrimitive::U8 => {
                write_data_using_u_number!(u8);
            },
            ToolboxIdlProgramDefPrimitive::U16 => {
                write_data_using_u_number!(u16);
            },
            ToolboxIdlProgramDefPrimitive::U32 => {
                write_data_using_u_number!(u32);
            },
            ToolboxIdlProgramDefPrimitive::U64 => {
                write_data_using_u_number!(u64);
            },
            ToolboxIdlProgramDefPrimitive::U128 => {
                let value_integer = idl_as_u128_or_else(value, context)?;
                data.extend_from_slice(bytemuck::bytes_of::<u128>(
                    &value_integer,
                ));
            },
            ToolboxIdlProgramDefPrimitive::I8 => {
                write_data_using_i_number!(i8);
            },
            ToolboxIdlProgramDefPrimitive::I16 => {
                write_data_using_i_number!(i16);
            },
            ToolboxIdlProgramDefPrimitive::I32 => {
                write_data_using_i_number!(i32);
            },
            ToolboxIdlProgramDefPrimitive::I64 => {
                write_data_using_i_number!(i64);
            },
            ToolboxIdlProgramDefPrimitive::I128 => {
                let value_integer = idl_as_i128_or_else(value, context)?;
                data.extend_from_slice(bytemuck::bytes_of::<i128>(
                    &value_integer,
                ));
            },
            ToolboxIdlProgramDefPrimitive::F32 => {
                let value_floating = idl_as_f64_or_else(value, context)? as f32;
                data.extend_from_slice(bytemuck::bytes_of::<f32>(
                    &value_floating,
                ));
            },
            ToolboxIdlProgramDefPrimitive::F64 => {
                let value_floating = idl_as_f64_or_else(value, context)?;
                data.extend_from_slice(bytemuck::bytes_of::<f64>(
                    &value_floating,
                ));
            },
            ToolboxIdlProgramDefPrimitive::Bytes => {
                let value_bytes = idl_as_bytes_or_else(value, context)?;
                let value_length = u32::try_from(value_bytes.len()).unwrap();
                data.extend_from_slice(bytemuck::bytes_of::<u32>(
                    &value_length,
                ));
                data.extend_from_slice(&value_bytes);
            },
            ToolboxIdlProgramDefPrimitive::Boolean => {
                data.push(if idl_as_bool_or_else(value, context)? {
                    1
                } else {
                    0
                });
            },
            ToolboxIdlProgramDefPrimitive::String => {
                let value_str = idl_as_str_or_else(value, context)?;
                let value_length = u32::try_from(value_str.len()).unwrap();
                data.extend_from_slice(bytemuck::bytes_of::<u32>(
                    &value_length,
                ));
                data.extend_from_slice(value_str.as_bytes());
            },
            ToolboxIdlProgramDefPrimitive::PublicKey => {
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
