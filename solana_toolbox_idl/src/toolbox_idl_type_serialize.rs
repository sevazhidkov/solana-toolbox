use std::str::FromStr;

use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_type::ToolboxIdlType;
use crate::toolbox_idl_type::ToolboxIdlTypePrimitiveKind;
use crate::toolbox_idl_utils::idl_as_array_or_else;
use crate::toolbox_idl_utils::idl_as_bool_or_else;
use crate::toolbox_idl_utils::idl_as_f64_or_else;
use crate::toolbox_idl_utils::idl_as_i128_or_else;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_as_str_or_else;
use crate::toolbox_idl_utils::idl_as_u128_or_else;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;

impl ToolboxIdl {
    pub(crate) fn type_serialize(
        &self,
        idl_type: &ToolboxIdlType,
        value: &Value,
        data: &mut Vec<u8>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        idl_type_serialize(self, idl_type, value, data, breadcrumbs)
    }
}

fn idl_type_serialize(
    idl: &ToolboxIdl,
    idl_type: &ToolboxIdlType,
    value: &Value,
    data: &mut Vec<u8>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(), ToolboxIdlError> {
    match idl_type {
        ToolboxIdlType::Defined { name } => {
            idl_type_serialize_defined(
                idl,
                name,
                value,
                data,
                &breadcrumbs.with_idl(name),
            )
        },
        ToolboxIdlType::Option { content } => {
            idl_type_serialize_option(
                idl,
                content,
                value,
                data,
                &breadcrumbs.with_idl("option"),
            )
        },
        ToolboxIdlType::Vec { items } => {
            idl_type_serialize_vec(
                idl,
                items,
                value,
                data,
                &breadcrumbs.with_idl("vec"),
            )
        },
        ToolboxIdlType::Array { length, items } => {
            idl_type_serialize_array(
                idl,
                *length,
                items,
                value,
                data,
                &breadcrumbs.with_idl("array"),
            )
        },
        ToolboxIdlType::Struct { fields } => {
            idl_type_serialize_struct(
                idl,
                fields,
                value,
                data,
                &breadcrumbs.with_idl("struct"),
            )
        },
        ToolboxIdlType::Enum { variants } => {
            idl_type_serialize_enum(
                variants,
                value,
                data,
                &breadcrumbs.with_idl("enum"),
            )
        },
        ToolboxIdlType::Primitive { kind } => {
            idl_type_serialize_primitive(kind, value, data, breadcrumbs)
        },
    }
}

fn idl_type_serialize_defined(
    idl: &ToolboxIdl,
    idl_defined_name: &str,
    value: &Value,
    data: &mut Vec<u8>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(), ToolboxIdlError> {
    let idl_defined_type = idl_map_get_key_or_else(
        &idl.program_types,
        idl_defined_name,
        &breadcrumbs.as_idl("$program_types"),
    )?;
    idl_type_serialize(
        idl,
        idl_defined_type,
        value,
        data,
        breadcrumbs,
    )
}

fn idl_type_serialize_option(
    idl: &ToolboxIdl,
    idl_option_content: &ToolboxIdlType,
    value: &Value,
    data: &mut Vec<u8>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(), ToolboxIdlError> {
    if value.is_null() {
        data.push(0);
        Ok(())
    } else {
        data.push(1);
        idl_type_serialize(idl, idl_option_content, value, data, breadcrumbs)
    }
}

fn idl_type_serialize_vec(
    idl: &ToolboxIdl,
    idl_vec_items: &ToolboxIdlType,
    value: &Value,
    data: &mut Vec<u8>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(), ToolboxIdlError> {
    let value_array = idl_as_array_or_else(value, &breadcrumbs.as_val("vec"))?;
    let value_length = u32::try_from(value_array.len()).unwrap();
    data.extend_from_slice(bytemuck::bytes_of::<u32>(&value_length));
    for (index, value_item) in value_array.iter().enumerate() {
        idl_type_serialize(
            idl,
            idl_vec_items,
            value_item,
            data,
            &breadcrumbs.with_val(&format!("[{}]", index)),
        )?;
    }
    Ok(())
}

fn idl_type_serialize_array(
    idl: &ToolboxIdl,
    idl_array_length: u32,
    idl_array_items: &ToolboxIdlType,
    value: &Value,
    data: &mut Vec<u8>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(), ToolboxIdlError> {
    let value_array =
        idl_as_array_or_else(value, &breadcrumbs.as_val("array"))?;
    if value_array.len() != idl_array_length as usize {
        return idl_err(
            &format!(
                "value array is not the correct size: expected {} items, found {} items",
                idl_array_length,
                value_array.len()
            ),
            &breadcrumbs.as_idl("value array"),
        );
    }
    for (index, value_item) in value_array.iter().enumerate() {
        idl_type_serialize(
            idl,
            idl_array_items,
            value_item,
            data,
            &breadcrumbs.with_val(&format!("[{}]", index)),
        )?;
    }
    Ok(())
}

fn idl_type_serialize_struct(
    idl: &ToolboxIdl,
    idl_struct_fields: &[(String, ToolboxIdlType)],
    value: &Value,
    data: &mut Vec<u8>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(), ToolboxIdlError> {
    let value_object =
        idl_as_object_or_else(value, &breadcrumbs.as_val("struct"))?;
    for (idl_struct_field_name, idl_struct_field_type) in idl_struct_fields {
        let breadcrumbs = &breadcrumbs.with_idl(idl_struct_field_name);
        let value_field = idl_object_get_key_or_else(
            value_object,
            idl_struct_field_name,
            &breadcrumbs.val(),
        )?;
        idl_type_serialize(
            idl,
            idl_struct_field_type,
            value_field,
            data,
            &breadcrumbs.with_val(idl_struct_field_name),
        )?;
    }
    Ok(())
}

fn idl_type_serialize_enum(
    idl_enum_variants: &[String],
    value: &Value,
    data: &mut Vec<u8>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(), ToolboxIdlError> {
    let value_string = idl_as_str_or_else(value, &breadcrumbs.as_val("enum"))?;
    for (value_enum, idl_enum_variant) in idl_enum_variants.iter().enumerate() {
        if idl_enum_variant == value_string {
            data.push(u8::try_from(value_enum).unwrap());
            return Ok(());
        }
    }
    idl_err("could not find matching enum", &breadcrumbs.as_val(value_string))
}

fn idl_type_serialize_primitive(
    idl_primitive_kind: &ToolboxIdlTypePrimitiveKind,
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
            data.extend_from_slice(bytemuck::bytes_of::<$type>(&value_typed));
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
            data.extend_from_slice(bytemuck::bytes_of::<$type>(&value_typed));
        };
    }
    match idl_primitive_kind {
        ToolboxIdlTypePrimitiveKind::U8 => {
            write_data_using_u_number!(u8);
        },
        ToolboxIdlTypePrimitiveKind::U16 => {
            write_data_using_u_number!(u16);
        },
        ToolboxIdlTypePrimitiveKind::U32 => {
            write_data_using_u_number!(u32);
        },
        ToolboxIdlTypePrimitiveKind::U64 => {
            write_data_using_u_number!(u64);
        },
        ToolboxIdlTypePrimitiveKind::U128 => {
            let value_integer = idl_as_u128_or_else(value, context)?;
            data.extend_from_slice(bytemuck::bytes_of::<u128>(&value_integer));
        },
        ToolboxIdlTypePrimitiveKind::I8 => {
            write_data_using_i_number!(i8);
        },
        ToolboxIdlTypePrimitiveKind::I16 => {
            write_data_using_i_number!(i16);
        },
        ToolboxIdlTypePrimitiveKind::I32 => {
            write_data_using_i_number!(i32);
        },
        ToolboxIdlTypePrimitiveKind::I64 => {
            write_data_using_i_number!(i64);
        },
        ToolboxIdlTypePrimitiveKind::I128 => {
            let value_integer = idl_as_i128_or_else(value, context)?;
            data.extend_from_slice(bytemuck::bytes_of::<i128>(&value_integer));
        },
        ToolboxIdlTypePrimitiveKind::F32 => {
            let value_floating = idl_as_f64_or_else(value, context)? as f32;
            data.extend_from_slice(bytemuck::bytes_of::<f32>(&value_floating));
        },
        ToolboxIdlTypePrimitiveKind::F64 => {
            let value_floating = idl_as_f64_or_else(value, context)?;
            data.extend_from_slice(bytemuck::bytes_of::<f64>(&value_floating));
        },
        ToolboxIdlTypePrimitiveKind::Boolean => {
            data.push(if idl_as_bool_or_else(value, context)? { 1 } else { 0 });
        },
        ToolboxIdlTypePrimitiveKind::String => {
            let value_str = idl_as_str_or_else(value, context)?;
            let value_length = u32::try_from(value_str.len()).unwrap();
            data.extend_from_slice(bytemuck::bytes_of::<u32>(&value_length));
            data.extend_from_slice(value_str.as_bytes());
        },
        ToolboxIdlTypePrimitiveKind::PublicKey => {
            let value_str = idl_as_str_or_else(value, context)?;
            let value_pubkey = Pubkey::from_str(value_str).map_err(|err| {
                ToolboxIdlError::InvalidPubkey {
                    parsing: err,
                    context: context.clone(),
                }
            })?;
            data.extend_from_slice(bytemuck::bytes_of::<Pubkey>(&value_pubkey));
        },
    };
    Ok(())
}
