use std::cmp::max;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullEnumVariant;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;
use crate::toolbox_idl_type_prefix::ToolboxIdlTypePrefix;
use crate::toolbox_idl_type_primitive::ToolboxIdlTypePrimitive;
use crate::toolbox_idl_utils::idl_pubkey_from_bytes_at;
use crate::toolbox_idl_utils::idl_slice_from_bytes;
use crate::toolbox_idl_utils::idl_u32_from_bytes_at;
use crate::toolbox_idl_utils::idl_u8_from_bytes_at;

impl ToolboxIdlTypeFull {
    pub fn try_deserialize(
        &self,
        data: &[u8],
        data_offset: usize,
        // TODO (FAR) - support deserializing bytes into custom stuff like base64 ?
    ) -> Result<(usize, Value)> {
        match self {
            ToolboxIdlTypeFull::Option { prefix, content } => {
                ToolboxIdlTypeFull::try_deserialize_option(
                    prefix,
                    content,
                    data,
                    data_offset,
                )
            },
            ToolboxIdlTypeFull::Vec { prefix, items } => {
                ToolboxIdlTypeFull::try_deserialize_vec(
                    prefix,
                    items,
                    data,
                    data_offset,
                )
            },
            ToolboxIdlTypeFull::Array { items, length } => {
                ToolboxIdlTypeFull::try_deserialize_array(
                    items,
                    length,
                    data,
                    data_offset,
                )
            },
            ToolboxIdlTypeFull::Struct { fields } => {
                ToolboxIdlTypeFull::try_deserialize_struct(
                    fields,
                    data,
                    data_offset,
                )
            },
            ToolboxIdlTypeFull::Enum { prefix, variants } => {
                ToolboxIdlTypeFull::try_deserialize_enum(
                    prefix,
                    variants,
                    data,
                    data_offset,
                )
            },
            ToolboxIdlTypeFull::Padded {
                size_bytes,
                content,
            } => ToolboxIdlTypeFull::try_deserialize_padded(
                size_bytes,
                content,
                data,
                data_offset,
            ),
            ToolboxIdlTypeFull::Const { literal } => {
                Err(anyhow!("Can't use a const literal directly: {}", literal))
            },
            ToolboxIdlTypeFull::Primitive { primitive } => {
                ToolboxIdlTypeFull::try_deserialize_primitive(
                    primitive,
                    data,
                    data_offset,
                )
            },
        }
    }

    fn try_deserialize_option(
        option_prefix: &ToolboxIdlTypePrefix,
        option_content: &ToolboxIdlTypeFull,
        data: &[u8],
        data_offset: usize,
    ) -> Result<(usize, Value)> {
        let data_flag = option_prefix.from_bytes_at(data, data_offset)?;
        let mut data_size = option_prefix.size_of();
        if data_flag > 0 {
            let (data_content_size, data_content) = option_content
                .try_deserialize(data, data_offset + data_size)?;
            data_size += data_content_size;
            Ok((data_size, data_content))
        } else {
            Ok((data_size, Value::Null))
        }
    }

    fn try_deserialize_vec(
        vec_prefix: &ToolboxIdlTypePrefix,
        vec_items: &ToolboxIdlTypeFull,
        data: &[u8],
        data_offset: usize,
    ) -> Result<(usize, Value)> {
        let data_length = vec_prefix.from_bytes_at(data, data_offset)?;
        let mut data_size = vec_prefix.size_of();
        let mut data_items = vec![];
        for index in 0..data_length {
            let (data_item_size, data_item) = vec_items
                .try_deserialize(data, data_offset + data_size)
                .with_context(|| format!("Decode Vec Item: {}", index))?;
            data_size += data_item_size;
            data_items.push(data_item);
        }
        Ok((data_size, json!(data_items)))
    }

    fn try_deserialize_array(
        array_items: &ToolboxIdlTypeFull,
        array_length: &u64,
        data: &[u8],
        data_offset: usize,
    ) -> Result<(usize, Value)> {
        let array_length = usize::try_from(*array_length)?;
        let mut data_size = 0;
        let mut data_items = vec![];
        for index in 0..array_length {
            let (data_item_size, data_item) = array_items
                .try_deserialize(data, data_offset + data_size)
                .with_context(|| format!("Decode Array Item: {}", index))?;
            data_size += data_item_size;
            data_items.push(data_item);
        }
        Ok((data_size, json!(data_items)))
    }

    fn try_deserialize_struct(
        struct_fields: &ToolboxIdlTypeFullFields,
        data: &[u8],
        data_offset: usize,
    ) -> Result<(usize, Value)> {
        struct_fields.try_deserialize(data, data_offset)
    }

    fn try_deserialize_enum(
        enum_prefix: &ToolboxIdlTypePrefix,
        enum_variants: &[ToolboxIdlTypeFullEnumVariant],
        data: &[u8],
        data_offset: usize,
    ) -> Result<(usize, Value)> {
        let data_code = enum_prefix.from_bytes_at(data, data_offset)?;
        for enum_variant in enum_variants {
            if enum_variant.code == data_code {
                let mut data_size = enum_prefix.size_of();
                let (data_fields_size, data_fields) = enum_variant
                    .fields
                    .try_deserialize(data, data_offset + data_size)
                    .with_context(|| {
                        format!(
                            "Decode Enum Variant Name: {}",
                            enum_variant.name
                        )
                    })?;
                data_size += data_fields_size;
                if data_fields.is_null() {
                    return Ok((data_size, json!(enum_variant.name)));
                } else {
                    return Ok((
                        data_size,
                        json!({ enum_variant.name.to_string(): data_fields }),
                    ));
                }
            }
        }
        Err(anyhow!(
            "Unknown enum code: {}, known variants: {:?}",
            data_code,
            enum_variants
                .iter()
                .map(|enum_variant| format!(
                    "{} ({})",
                    enum_variant.name, enum_variant.code
                ))
                .collect::<Vec<_>>(),
        ))
    }

    fn try_deserialize_padded(
        padded_size_bytes: &u64,
        padded_content: &ToolboxIdlTypeFull,
        data: &[u8],
        data_offset: usize,
    ) -> Result<(usize, Value)> {
        let padded_size_bytes = usize::try_from(*padded_size_bytes)?;
        let (data_content_size, data_content) =
            padded_content.try_deserialize(data, data_offset)?;
        Ok((max(data_content_size, padded_size_bytes), data_content))
    }

    fn try_deserialize_primitive(
        primitive: &ToolboxIdlTypePrimitive,
        data: &[u8],
        data_offset: usize,
    ) -> Result<(usize, Value)> {
        Ok(match primitive {
            ToolboxIdlTypePrimitive::U8 => {
                let int = idl_u8_from_bytes_at(data, data_offset)?;
                (std::mem::size_of_val(&int), json!(int))
            },
            ToolboxIdlTypePrimitive::U16 => {
                let size = std::mem::size_of::<u16>();
                let slice = idl_slice_from_bytes(data, data_offset, size)?;
                let num = u16::from_le_bytes(slice.try_into()?);
                (size, json!(num))
            },
            ToolboxIdlTypePrimitive::U32 => {
                let size = std::mem::size_of::<u32>();
                let slice = idl_slice_from_bytes(data, data_offset, size)?;
                let num = u32::from_le_bytes(slice.try_into()?);
                (size, json!(num))
            },
            ToolboxIdlTypePrimitive::U64 => {
                let size = std::mem::size_of::<u64>();
                let slice = idl_slice_from_bytes(data, data_offset, size)?;
                let num = u64::from_le_bytes(slice.try_into()?);
                (size, json!(num))
            },
            ToolboxIdlTypePrimitive::U128 => {
                let size = std::mem::size_of::<u128>();
                let slice = idl_slice_from_bytes(data, data_offset, size)?;
                let num = u128::from_le_bytes(slice.try_into()?);
                (size, json!(num))
            },
            ToolboxIdlTypePrimitive::I8 => {
                let size = std::mem::size_of::<i8>();
                let slice = idl_slice_from_bytes(data, data_offset, size)?;
                let num = i8::from_le_bytes(slice.try_into()?);
                (size, json!(num))
            },
            ToolboxIdlTypePrimitive::I16 => {
                let size = std::mem::size_of::<i16>();
                let slice = idl_slice_from_bytes(data, data_offset, size)?;
                let num = i16::from_le_bytes(slice.try_into()?);
                (size, json!(num))
            },
            ToolboxIdlTypePrimitive::I32 => {
                let size = std::mem::size_of::<i32>();
                let slice = idl_slice_from_bytes(data, data_offset, size)?;
                let num = i32::from_le_bytes(slice.try_into()?);
                (size, json!(num))
            },
            ToolboxIdlTypePrimitive::I64 => {
                let size = std::mem::size_of::<i64>();
                let slice = idl_slice_from_bytes(data, data_offset, size)?;
                let num = i64::from_le_bytes(slice.try_into()?);
                (size, json!(num))
            },
            ToolboxIdlTypePrimitive::I128 => {
                let size = std::mem::size_of::<i128>();
                let slice = idl_slice_from_bytes(data, data_offset, size)?;
                let num = i128::from_le_bytes(slice.try_into()?);
                (size, json!(num))
            },
            ToolboxIdlTypePrimitive::F32 => {
                let size = std::mem::size_of::<f32>();
                let slice = idl_slice_from_bytes(data, data_offset, size)?;
                let num = f32::from_le_bytes(slice.try_into()?);
                (size, json!(num))
            },
            ToolboxIdlTypePrimitive::F64 => {
                let size = std::mem::size_of::<f64>();
                let slice = idl_slice_from_bytes(data, data_offset, size)?;
                let num = f64::from_le_bytes(slice.try_into()?);
                (size, json!(num))
            },
            ToolboxIdlTypePrimitive::Boolean => {
                let data_flag = idl_u8_from_bytes_at(data, data_offset)?;
                let data_size = std::mem::size_of_val(&data_flag);
                (data_size, json!(data_flag != 0))
            },
            ToolboxIdlTypePrimitive::String => {
                let data_length = idl_u32_from_bytes_at(data, data_offset)?;
                let mut data_size = std::mem::size_of_val(&data_length);
                let data_bytes = idl_slice_from_bytes(
                    data,
                    data_offset + data_size,
                    usize::try_from(data_length)?,
                )?;
                data_size += data_bytes.len();
                let data_string = String::from_utf8(data_bytes.to_vec())?;
                (data_size, json!(data_string))
            },
            ToolboxIdlTypePrimitive::PublicKey => {
                let data_pubkey = idl_pubkey_from_bytes_at(data, data_offset)?;
                let data_size = std::mem::size_of_val(&data_pubkey);
                (data_size, json!(data_pubkey.to_string()))
            },
        })
    }
}

impl ToolboxIdlTypeFullFields {
    pub fn try_deserialize(
        &self,
        data: &[u8],
        data_offset: usize,
    ) -> Result<(usize, Value)> {
        Ok(match self {
            ToolboxIdlTypeFullFields::Named(fields) => {
                let mut data_size = 0;
                let mut data_fields = Map::new();
                for field in fields {
                    let (data_field_size, data_field) = field
                        .type_full
                        .try_deserialize(data, data_offset + data_size)
                        .with_context(|| {
                            format!("Decode Field: {}", field.name)
                        })?;
                    data_size += data_field_size;
                    data_fields.insert(field.name.to_string(), data_field);
                }
                (data_size, json!(data_fields))
            },
            ToolboxIdlTypeFullFields::Unnamed(fields) => {
                let mut data_size = 0;
                let mut data_fields = vec![];
                for (index, field) in fields.iter().enumerate() {
                    let (data_field_size, data_field) = field
                        .type_full
                        .try_deserialize(data, data_offset + data_size)
                        .with_context(|| format!("Decode Field: {}", index))?;
                    data_size += data_field_size;
                    data_fields.push(data_field);
                }
                (data_size, json!(data_fields))
            },
            ToolboxIdlTypeFullFields::None => (0, Value::Null),
        })
    }
}
