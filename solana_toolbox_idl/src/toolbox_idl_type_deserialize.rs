use serde_json::Map;
use serde_json::Number;
use serde_json::Value;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_type::ToolboxIdlType;
use crate::toolbox_idl_type::ToolboxIdlTypePrimitiveKind;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_f32_from_bytes_at;
use crate::toolbox_idl_utils::idl_f64_from_bytes_at;
use crate::toolbox_idl_utils::idl_i128_from_bytes_at;
use crate::toolbox_idl_utils::idl_i16_from_bytes_at;
use crate::toolbox_idl_utils::idl_i32_from_bytes_at;
use crate::toolbox_idl_utils::idl_i64_from_bytes_at;
use crate::toolbox_idl_utils::idl_i8_from_bytes_at;
use crate::toolbox_idl_utils::idl_pubkey_from_bytes_at;
use crate::toolbox_idl_utils::idl_slice_from_bytes;
use crate::toolbox_idl_utils::idl_u128_from_bytes_at;
use crate::toolbox_idl_utils::idl_u16_from_bytes_at;
use crate::toolbox_idl_utils::idl_u32_from_bytes_at;
use crate::toolbox_idl_utils::idl_u64_from_bytes_at;
use crate::toolbox_idl_utils::idl_u8_from_bytes_at;

impl ToolboxIdl {
    pub(crate) fn type_deserialize(
        &self,
        idl_type: &Value,
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        // TODO - remove
        let dada = self.parse_type(idl_type, breadcrumbs)?;
        idl_type_deserialize(self, &dada, data, data_offset, breadcrumbs)
    }
}

fn idl_type_deserialize(
    idl: &ToolboxIdl,
    idl_type: &ToolboxIdlType,
    data: &[u8],
    data_offset: usize,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(usize, Value), ToolboxIdlError> {
    match idl_type {
        ToolboxIdlType::Defined { name, lookup } => {
            idl_type_deserialize(
                idl,
                lookup,
                data,
                data_offset,
                &breadcrumbs.with_idl(name),
            )
        },
        ToolboxIdlType::Option { content } => {
            idl_type_deserialize_option(
                idl,
                content,
                data,
                data_offset,
                &breadcrumbs.with_idl("option"),
            )
        },
        ToolboxIdlType::Vec { items } => {
            idl_type_deserialize_vec(
                idl,
                items,
                data,
                data_offset,
                &breadcrumbs.with_idl("vec"),
            )
        },
        ToolboxIdlType::Array { length, items } => {
            idl_type_deserialize_array(
                idl,
                *length,
                items,
                data,
                data_offset,
                &breadcrumbs.with_idl("array"),
            )
        },
        ToolboxIdlType::Struct { fields } => {
            idl_type_deserialize_struct(
                idl,
                fields,
                data,
                data_offset,
                &breadcrumbs.with_idl("struct"),
            )
        },
        ToolboxIdlType::Enum { variants } => {
            idl_type_deserialize_enum(
                variants,
                data,
                data_offset,
                &breadcrumbs.with_idl("enum"),
            )
        },
        ToolboxIdlType::Primitive { kind } => {
            idl_type_deserialize_primitive(kind, data, data_offset, breadcrumbs)
        },
    }
}

fn idl_type_deserialize_option(
    idl: &ToolboxIdl,
    idl_option_content: &ToolboxIdlType,
    data: &[u8],
    data_offset: usize,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(usize, Value), ToolboxIdlError> {
    let data_flag =
        idl_u8_from_bytes_at(data, data_offset, &breadcrumbs.as_val("flag"))?;
    let mut data_size = size_of_val(&data_flag);
    if data_flag > 0 {
        let (data_content_size, data_content_value) = idl_type_deserialize(
            idl,
            idl_option_content,
            data,
            data_offset + data_size,
            breadcrumbs,
        )?;
        data_size += data_content_size;
        Ok((data_size, data_content_value))
    } else {
        Ok((data_size, Value::Null))
    }
}

fn idl_type_deserialize_vec(
    idl: &ToolboxIdl,
    idl_vec_items: &ToolboxIdlType,
    data: &[u8],
    data_offset: usize,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(usize, Value), ToolboxIdlError> {
    let data_length = idl_u32_from_bytes_at(
        data,
        data_offset,
        &breadcrumbs.as_val("length"),
    )?;
    let mut data_size = size_of_val(&data_length);
    let mut data_items = vec![];
    for index in 0..data_length {
        let (data_item_size, data_item_value) = idl_type_deserialize(
            idl,
            idl_vec_items,
            data,
            data_offset + data_size,
            &breadcrumbs.with_val(&format!("[{}]", index)),
        )?;
        data_size += data_item_size;
        data_items.push(data_item_value);
    }
    Ok((data_size, Value::Array(data_items)))
}

fn idl_type_deserialize_array(
    idl: &ToolboxIdl,
    idl_array_length: u32,
    idl_array_items: &ToolboxIdlType,
    data: &[u8],
    data_offset: usize,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(usize, Value), ToolboxIdlError> {
    let mut data_size = 0;
    let mut data_items = vec![];
    for index in 0..idl_array_length {
        let (data_item_size, data_item_value) = idl_type_deserialize(
            idl,
            idl_array_items,
            data,
            data_offset + data_size,
            &breadcrumbs.with_val(&format!("[{}]", index)),
        )?;
        data_size += data_item_size;
        data_items.push(data_item_value);
    }
    Ok((data_size, Value::Array(data_items)))
}

fn idl_type_deserialize_struct(
    idl: &ToolboxIdl,
    idl_struct_fields: &[(String, ToolboxIdlType)],
    data: &[u8],
    data_offset: usize,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(usize, Value), ToolboxIdlError> {
    let mut data_size = 0;
    let mut data_fields = Map::new();
    for (idl_struct_field_name, idl_struct_field_type) in idl_struct_fields {
        let breadcrumbs = &breadcrumbs.with_idl(idl_struct_field_name);
        let (data_field_size, data_field_value) = idl_type_deserialize(
            idl,
            idl_struct_field_type,
            data,
            data_offset + data_size,
            &breadcrumbs.with_val(idl_struct_field_name),
        )?;
        data_size += data_field_size;
        data_fields.insert(idl_struct_field_name.to_string(), data_field_value);
    }
    Ok((data_size, Value::Object(data_fields)))
}

fn idl_type_deserialize_enum(
    idl_enum_variants: &[String],
    data: &[u8],
    data_offset: usize,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(usize, Value), ToolboxIdlError> {
    let data_enum =
        idl_u8_from_bytes_at(data, data_offset, &breadcrumbs.as_val("enum"))?;
    let data_index = usize::from(data_enum);
    if data_index >= idl_enum_variants.len() {
        return idl_err(
            &format!("Invalid enum value: {}", data_index),
            &breadcrumbs.as_idl("variants"),
        );
    }
    let idl_enum_variant = &idl_enum_variants[data_index];
    Ok((size_of_val(&data_enum), Value::String(idl_enum_variant.to_string())))
}

fn idl_type_deserialize_primitive(
    idl_primitive_kind: &ToolboxIdlTypePrimitiveKind,
    data: &[u8],
    data_offset: usize,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(usize, Value), ToolboxIdlError> {
    let context = &breadcrumbs.val();
    Ok(match idl_primitive_kind {
        ToolboxIdlTypePrimitiveKind::U8 => {
            let int = idl_u8_from_bytes_at(data, data_offset, context)?;
            (size_of_val(&int), Value::Number(Number::from(int)))
        },
        ToolboxIdlTypePrimitiveKind::U16 => {
            let int = idl_u16_from_bytes_at(data, data_offset, context)?;
            (size_of_val(&int), Value::Number(Number::from(int)))
        },
        ToolboxIdlTypePrimitiveKind::U32 => {
            let int = idl_u32_from_bytes_at(data, data_offset, context)?;
            (size_of_val(&int), Value::Number(Number::from(int)))
        },
        ToolboxIdlTypePrimitiveKind::U64 => {
            let int = idl_u64_from_bytes_at(data, data_offset, context)?;
            (size_of_val(&int), Value::Number(Number::from(int)))
        },
        ToolboxIdlTypePrimitiveKind::U128 => {
            let int = idl_u128_from_bytes_at(data, data_offset, context)?;
            (
                size_of_val(&int),
                Value::Number(
                    Number::from_u128(int).unwrap_or(Number::from(0)),
                ),
            )
        },
        ToolboxIdlTypePrimitiveKind::I8 => {
            let int = idl_i8_from_bytes_at(data, data_offset, context)?;
            (size_of_val(&int), Value::Number(Number::from(int)))
        },
        ToolboxIdlTypePrimitiveKind::I16 => {
            let int = idl_i16_from_bytes_at(data, data_offset, context)?;
            (size_of_val(&int), Value::Number(Number::from(int)))
        },
        ToolboxIdlTypePrimitiveKind::I32 => {
            let int = idl_i32_from_bytes_at(data, data_offset, context)?;
            (size_of_val(&int), Value::Number(Number::from(int)))
        },
        ToolboxIdlTypePrimitiveKind::I64 => {
            let int = idl_i64_from_bytes_at(data, data_offset, context)?;
            (size_of_val(&int), Value::Number(Number::from(int)))
        },
        ToolboxIdlTypePrimitiveKind::I128 => {
            let int = idl_i128_from_bytes_at(data, data_offset, context)?;
            (
                size_of_val(&int),
                Value::Number(
                    Number::from_i128(int).unwrap_or(Number::from(0)),
                ),
            )
        },
        ToolboxIdlTypePrimitiveKind::F32 => {
            let float =
                idl_f32_from_bytes_at(data, data_offset, context)? as f64;
            (
                size_of_val(&float),
                Value::Number(
                    Number::from_f64(float).unwrap_or(Number::from(0)),
                ),
            )
        },
        ToolboxIdlTypePrimitiveKind::F64 => {
            let float = idl_f64_from_bytes_at(data, data_offset, context)?;
            (
                size_of_val(&float),
                Value::Number(
                    Number::from_f64(float).unwrap_or(Number::from(0)),
                ),
            )
        },
        ToolboxIdlTypePrimitiveKind::Boolean => {
            let data_flag = idl_u8_from_bytes_at(data, data_offset, context)?;
            let data_size = size_of_val(&data_flag);
            (data_size, Value::Bool(data_flag != 0))
        },
        ToolboxIdlTypePrimitiveKind::String => {
            let data_length =
                idl_u32_from_bytes_at(data, data_offset, context)?;
            let mut data_size = size_of_val(&data_length);
            let data_bytes = idl_slice_from_bytes(
                data,
                data_offset + data_size,
                usize::try_from(data_length).map_err(|err| {
                    ToolboxIdlError::InvalidInteger {
                        conversion: err,
                        context: context.clone(),
                    }
                })?,
                context,
            )?;
            data_size += data_bytes.len();
            let data_string =
                String::from_utf8(data_bytes.to_vec()).map_err(|err| {
                    ToolboxIdlError::InvalidString {
                        parsing: err,
                        context: context.clone(),
                    }
                })?;
            (data_size, Value::String(data_string))
        },
        ToolboxIdlTypePrimitiveKind::PublicKey => {
            let data_pubkey =
                idl_pubkey_from_bytes_at(data, data_offset, context)?;
            let data_size = size_of_val(&data_pubkey);
            (data_size, Value::String(data_pubkey.to_string()))
        },
    })
}
