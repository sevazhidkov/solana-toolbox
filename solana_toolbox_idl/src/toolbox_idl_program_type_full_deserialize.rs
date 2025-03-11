use serde_json::Map;
use serde_json::Number;
use serde_json::Value;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_type_full::ToolboxIdlProgramTypeFull;
use crate::toolbox_idl_program_type_full::ToolboxIdlProgramTypeFullFields;
use crate::toolbox_idl_program_type_primitive::ToolboxIdlProgramTypePrimitive;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_f32_from_bytes_at;
use crate::toolbox_idl_utils::idl_f64_from_bytes_at;
use crate::toolbox_idl_utils::idl_i128_from_bytes_at;
use crate::toolbox_idl_utils::idl_i16_from_bytes_at;
use crate::toolbox_idl_utils::idl_i32_from_bytes_at;
use crate::toolbox_idl_utils::idl_i64_from_bytes_at;
use crate::toolbox_idl_utils::idl_i8_from_bytes_at;
use crate::toolbox_idl_utils::idl_iter_get_scoped_values;
use crate::toolbox_idl_utils::idl_map_err_invalid_integer;
use crate::toolbox_idl_utils::idl_pubkey_from_bytes_at;
use crate::toolbox_idl_utils::idl_slice_from_bytes;
use crate::toolbox_idl_utils::idl_u128_from_bytes_at;
use crate::toolbox_idl_utils::idl_u16_from_bytes_at;
use crate::toolbox_idl_utils::idl_u32_from_bytes_at;
use crate::toolbox_idl_utils::idl_u64_from_bytes_at;
use crate::toolbox_idl_utils::idl_u8_from_bytes_at;

impl ToolboxIdlProgramTypeFull {
    pub fn try_deserialize(
        &self,
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        match self {
            ToolboxIdlProgramTypeFull::Option { content } => {
                ToolboxIdlProgramTypeFull::try_deserialize_option(
                    content,
                    data,
                    data_offset,
                    &breadcrumbs.with_idl("option"),
                )
            },
            ToolboxIdlProgramTypeFull::Vec { items } => {
                ToolboxIdlProgramTypeFull::try_deserialize_vec(
                    items,
                    data,
                    data_offset,
                    &breadcrumbs.with_idl("vec"),
                )
            },
            ToolboxIdlProgramTypeFull::Array { items, length } => {
                ToolboxIdlProgramTypeFull::try_deserialize_array(
                    items,
                    *length,
                    data,
                    data_offset,
                    &breadcrumbs.with_idl("array"),
                )
            },
            ToolboxIdlProgramTypeFull::Struct { fields } => {
                ToolboxIdlProgramTypeFull::try_deserialize_struct(
                    fields,
                    data,
                    data_offset,
                    &breadcrumbs.with_idl("struct"),
                )
            },
            ToolboxIdlProgramTypeFull::Enum { variants } => {
                ToolboxIdlProgramTypeFull::try_deserialize_enum(
                    variants,
                    data,
                    data_offset,
                    &breadcrumbs.with_idl("enum"),
                )
            },
            ToolboxIdlProgramTypeFull::Const { literal } => idl_err(
                &format!("Can't use a const literal directly: {:?}", literal),
                &breadcrumbs.idl(),
            ),
            ToolboxIdlProgramTypeFull::Primitive { primitive } => {
                ToolboxIdlProgramTypeFull::try_deserialize_primitive(
                    primitive,
                    data,
                    data_offset,
                    breadcrumbs,
                )
            },
        }
    }

    fn try_deserialize_option(
        option_content: &ToolboxIdlProgramTypeFull,
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        let data_flag = idl_u8_from_bytes_at(
            data,
            data_offset,
            &breadcrumbs.as_val("flag"),
        )?;
        let mut data_size = std::mem::size_of_val(&data_flag);
        if data_flag > 0 {
            let (data_content_size, data_content) = option_content
                .try_deserialize(data, data_offset + data_size, breadcrumbs)?;
            data_size += data_content_size;
            Ok((data_size, data_content))
        } else {
            Ok((data_size, Value::Null))
        }
    }

    fn try_deserialize_vec(
        vec_items: &ToolboxIdlProgramTypeFull,
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        let data_length = idl_u32_from_bytes_at(
            data,
            data_offset,
            &breadcrumbs.as_val("length"),
        )?;
        let mut data_size = std::mem::size_of_val(&data_length);
        let mut data_items = vec![];
        for (_, _, breadcrumbs) in idl_iter_get_scoped_values(
            &(0..data_length).collect::<Vec<u32>>(),
            breadcrumbs,
        )? {
            let (data_item_size, data_item) = vec_items.try_deserialize(
                data,
                data_offset + data_size,
                &breadcrumbs,
            )?;
            data_size += data_item_size;
            data_items.push(data_item);
        }
        Ok((data_size, Value::Array(data_items)))
    }

    fn try_deserialize_array(
        array_items: &ToolboxIdlProgramTypeFull,
        array_length: usize,
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        let mut data_size = 0;
        let mut data_items = vec![];
        for (_, _, breadcrumbs) in idl_iter_get_scoped_values(
            &(0..array_length).collect::<Vec<usize>>(),
            breadcrumbs,
        )? {
            let (data_item_size, data_item) = array_items.try_deserialize(
                data,
                data_offset + data_size,
                &breadcrumbs,
            )?;
            data_size += data_item_size;
            data_items.push(data_item);
        }
        Ok((data_size, Value::Array(data_items)))
    }

    fn try_deserialize_struct(
        struct_fields: &ToolboxIdlProgramTypeFullFields,
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        ToolboxIdlProgramTypeFullFields::try_deserialize(
            struct_fields,
            data,
            data_offset,
            breadcrumbs,
        )
    }

    fn try_deserialize_enum(
        enum_variants: &[(String, ToolboxIdlProgramTypeFullFields)],
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        let data_enum = idl_u8_from_bytes_at(
            data,
            data_offset,
            &breadcrumbs.as_val("enum"),
        )?;
        let data_index = usize::from(data_enum);
        if data_index >= enum_variants.len() {
            return idl_err(
                &format!("Invalid enum value: {}", data_index),
                &breadcrumbs.as_idl("variants"),
            );
        }
        let mut data_size = std::mem::size_of_val(&data_enum);
        let enum_variant = &enum_variants[data_index];
        let (data_fields_size, data_fields) =
            ToolboxIdlProgramTypeFullFields::try_deserialize(
                &enum_variant.1,
                data,
                data_offset + data_size,
                breadcrumbs,
            )?;
        data_size += data_fields_size;
        if data_fields.is_null() {
            Ok((data_size, Value::String(enum_variant.0.to_string())))
        } else {
            Ok((
                data_size,
                Value::Array(vec![
                    Value::String(enum_variant.0.to_string()),
                    data_fields,
                ]),
            ))
        }
    }

    fn try_deserialize_primitive(
        primitive: &ToolboxIdlProgramTypePrimitive,
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        let context = &breadcrumbs.val();
        Ok(match primitive {
            ToolboxIdlProgramTypePrimitive::U8 => {
                let int = idl_u8_from_bytes_at(data, data_offset, context)?;
                (
                    std::mem::size_of_val(&int),
                    Value::Number(Number::from(int)),
                )
            },
            ToolboxIdlProgramTypePrimitive::U16 => {
                let int = idl_u16_from_bytes_at(data, data_offset, context)?;
                (
                    std::mem::size_of_val(&int),
                    Value::Number(Number::from(int)),
                )
            },
            ToolboxIdlProgramTypePrimitive::U32 => {
                let int = idl_u32_from_bytes_at(data, data_offset, context)?;
                (
                    std::mem::size_of_val(&int),
                    Value::Number(Number::from(int)),
                )
            },
            ToolboxIdlProgramTypePrimitive::U64 => {
                let int = idl_u64_from_bytes_at(data, data_offset, context)?;
                (
                    std::mem::size_of_val(&int),
                    Value::Number(Number::from(int)),
                )
            },
            ToolboxIdlProgramTypePrimitive::U128 => {
                let int = idl_u128_from_bytes_at(data, data_offset, context)?;
                (
                    std::mem::size_of_val(&int),
                    Value::Number(
                        Number::from_u128(int).unwrap_or(Number::from(0)),
                    ),
                )
            },
            ToolboxIdlProgramTypePrimitive::I8 => {
                let int = idl_i8_from_bytes_at(data, data_offset, context)?;
                (
                    std::mem::size_of_val(&int),
                    Value::Number(Number::from(int)),
                )
            },
            ToolboxIdlProgramTypePrimitive::I16 => {
                let int = idl_i16_from_bytes_at(data, data_offset, context)?;
                (
                    std::mem::size_of_val(&int),
                    Value::Number(Number::from(int)),
                )
            },
            ToolboxIdlProgramTypePrimitive::I32 => {
                let int = idl_i32_from_bytes_at(data, data_offset, context)?;
                (
                    std::mem::size_of_val(&int),
                    Value::Number(Number::from(int)),
                )
            },
            ToolboxIdlProgramTypePrimitive::I64 => {
                let int = idl_i64_from_bytes_at(data, data_offset, context)?;
                (
                    std::mem::size_of_val(&int),
                    Value::Number(Number::from(int)),
                )
            },
            ToolboxIdlProgramTypePrimitive::I128 => {
                let int = idl_i128_from_bytes_at(data, data_offset, context)?;
                (
                    std::mem::size_of_val(&int),
                    Value::Number(
                        Number::from_i128(int).unwrap_or(Number::from(0)),
                    ),
                )
            },
            ToolboxIdlProgramTypePrimitive::F32 => {
                let float =
                    idl_f32_from_bytes_at(data, data_offset, context)? as f64;
                (
                    std::mem::size_of_val(&float),
                    Value::Number(
                        Number::from_f64(float).unwrap_or(Number::from(0)),
                    ),
                )
            },
            ToolboxIdlProgramTypePrimitive::F64 => {
                let float = idl_f64_from_bytes_at(data, data_offset, context)?;
                (
                    std::mem::size_of_val(&float),
                    Value::Number(
                        Number::from_f64(float).unwrap_or(Number::from(0)),
                    ),
                )
            },
            ToolboxIdlProgramTypePrimitive::Bytes => {
                let data_length =
                    idl_u32_from_bytes_at(data, data_offset, context)?;
                let mut data_size = std::mem::size_of_val(&data_length);
                let data_bytes = idl_slice_from_bytes(
                    data,
                    data_offset + data_size,
                    idl_map_err_invalid_integer(
                        usize::try_from(data_length),
                        context,
                    )?,
                    context,
                )?;
                data_size += data_bytes.len();
                let mut data = vec![];
                for data_byte in data_bytes {
                    data.push(Value::Number(Number::from(*data_byte)));
                }
                (data_size, Value::Array(data))
            },
            ToolboxIdlProgramTypePrimitive::Boolean => {
                let data_flag =
                    idl_u8_from_bytes_at(data, data_offset, context)?;
                let data_size = std::mem::size_of_val(&data_flag);
                (data_size, Value::Bool(data_flag != 0))
            },
            ToolboxIdlProgramTypePrimitive::String => {
                let data_length =
                    idl_u32_from_bytes_at(data, data_offset, context)?;
                let mut data_size = std::mem::size_of_val(&data_length);
                let data_bytes = idl_slice_from_bytes(
                    data,
                    data_offset + data_size,
                    idl_map_err_invalid_integer(
                        usize::try_from(data_length),
                        context,
                    )?,
                    context,
                )?;
                data_size += data_bytes.len();
                let data_string = String::from_utf8(data_bytes.to_vec())
                    .map_err(|err| ToolboxIdlError::InvalidString {
                        parsing: err,
                        context: context.clone(),
                    })?;
                (data_size, Value::String(data_string))
            },
            ToolboxIdlProgramTypePrimitive::PublicKey => {
                let data_pubkey =
                    idl_pubkey_from_bytes_at(data, data_offset, context)?;
                let data_size = std::mem::size_of_val(&data_pubkey);
                (data_size, Value::String(data_pubkey.to_string()))
            },
        })
    }
}

impl ToolboxIdlProgramTypeFullFields {
    fn try_deserialize(
        &self,
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        Ok(match self {
            ToolboxIdlProgramTypeFullFields::Named(fields) => {
                let mut data_size = 0;
                let mut data_fields = Map::new();
                for (field_name, field) in fields {
                    let breadcrumbs = &breadcrumbs.with_idl(field_name);
                    let (data_field_size, data_field) = field.try_deserialize(
                        data,
                        data_offset + data_size,
                        &breadcrumbs.with_val(field_name),
                    )?;
                    data_size += data_field_size;
                    data_fields.insert(field_name.to_string(), data_field);
                }
                (data_size, Value::Object(data_fields))
            },
            ToolboxIdlProgramTypeFullFields::Unamed(fields) => {
                let mut data_size = 0;
                let mut data_fields = vec![];
                for (_, field, breadcrumbs) in
                    idl_iter_get_scoped_values(fields, breadcrumbs)?
                {
                    let (data_field_size, data_field) = field.try_deserialize(
                        data,
                        data_offset + data_size,
                        &breadcrumbs,
                    )?;
                    data_size += data_field_size;
                    data_fields.push(data_field);
                }
                (data_size, Value::Array(data_fields))
            },
            ToolboxIdlProgramTypeFullFields::None => (0, Value::Null),
        })
    }
}
