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
            ToolboxIdlTypeFull::Option {
                prefix, content, ..
            } => ToolboxIdlTypeFull::try_deserialize_option(
                prefix,
                content,
                data,
                data_offset,
            ),
            ToolboxIdlTypeFull::Vec { prefix, items, .. } => {
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
            ToolboxIdlTypeFull::Struct { fields, .. } => {
                ToolboxIdlTypeFull::try_deserialize_struct(
                    fields,
                    data,
                    data_offset,
                )
            },
            ToolboxIdlTypeFull::Enum {
                prefix, variants, ..
            } => ToolboxIdlTypeFull::try_deserialize_enum(
                prefix,
                variants,
                data,
                data_offset,
            ),
            ToolboxIdlTypeFull::Padded {
                before,
                size,
                after,
                content,
            } => ToolboxIdlTypeFull::try_deserialize_padded(
                before,
                size,
                after,
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
        let mut data_size = option_prefix.size();
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
        let mut data_size = vec_prefix.size();
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
                let mut data_size = enum_prefix.size();
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

    // TODO - test padded mode ?
    fn try_deserialize_padded(
        padded_before: &Option<u64>,
        padded_size: &Option<u64>,
        padded_after: &Option<u64>,
        padded_content: &ToolboxIdlTypeFull,
        data: &[u8],
        data_offset: usize,
    ) -> Result<(usize, Value)> {
        let data_offset_before =
            data_offset + usize::try_from(padded_before.unwrap_or(0))?;
        let (data_content_size, data_content) =
            padded_content.try_deserialize(data, data_offset_before)?;
        let data_offset_after = data_offset_before
            + max(
                usize::try_from(padded_size.unwrap_or(0))?,
                data_content_size,
            )
            + usize::try_from(padded_after.unwrap_or(0))?;
        let data_size = data_offset_after - data_offset_before;
        Ok((data_size, data_content))
    }

    fn try_deserialize_primitive(
        primitive: &ToolboxIdlTypePrimitive,
        data: &[u8],
        data_offset: usize,
    ) -> Result<(usize, Value)> {
        Ok(match primitive {
            ToolboxIdlTypePrimitive::U8 => {
                let data_size = std::mem::size_of::<u8>();
                let data_slice =
                    idl_slice_from_bytes(data, data_offset, data_size)?;
                let data_num = u8::from_le_bytes(data_slice.try_into()?);
                (data_size, json!(data_num))
            },
            ToolboxIdlTypePrimitive::U16 => {
                let data_size = std::mem::size_of::<u16>();
                let data_slice =
                    idl_slice_from_bytes(data, data_offset, data_size)?;
                let data_num = u16::from_le_bytes(data_slice.try_into()?);
                (data_size, json!(data_num))
            },
            ToolboxIdlTypePrimitive::U32 => {
                let data_size = std::mem::size_of::<u32>();
                let data_slice =
                    idl_slice_from_bytes(data, data_offset, data_size)?;
                let data_num = u32::from_le_bytes(data_slice.try_into()?);
                (data_size, json!(data_num))
            },
            ToolboxIdlTypePrimitive::U64 => {
                let data_size = std::mem::size_of::<u64>();
                let data_slice =
                    idl_slice_from_bytes(data, data_offset, data_size)?;
                let data_num = u64::from_le_bytes(data_slice.try_into()?);
                (data_size, json!(data_num))
            },
            ToolboxIdlTypePrimitive::U128 => {
                let data_size = std::mem::size_of::<u128>();
                let data_slice =
                    idl_slice_from_bytes(data, data_offset, data_size)?;
                let data_num = u128::from_le_bytes(data_slice.try_into()?);
                (data_size, json!(data_num))
            },
            ToolboxIdlTypePrimitive::I8 => {
                let data_size = std::mem::size_of::<i8>();
                let data_slice =
                    idl_slice_from_bytes(data, data_offset, data_size)?;
                let data_num = i8::from_le_bytes(data_slice.try_into()?);
                (data_size, json!(data_num))
            },
            ToolboxIdlTypePrimitive::I16 => {
                let data_size = std::mem::size_of::<i16>();
                let data_slice =
                    idl_slice_from_bytes(data, data_offset, data_size)?;
                let data_num = i16::from_le_bytes(data_slice.try_into()?);
                (data_size, json!(data_num))
            },
            ToolboxIdlTypePrimitive::I32 => {
                let data_size = std::mem::size_of::<i32>();
                let data_slice =
                    idl_slice_from_bytes(data, data_offset, data_size)?;
                let data_num = i32::from_le_bytes(data_slice.try_into()?);
                (data_size, json!(data_num))
            },
            ToolboxIdlTypePrimitive::I64 => {
                let data_size = std::mem::size_of::<i64>();
                let data_slice =
                    idl_slice_from_bytes(data, data_offset, data_size)?;
                let data_num = i64::from_le_bytes(data_slice.try_into()?);
                (data_size, json!(data_num))
            },
            ToolboxIdlTypePrimitive::I128 => {
                let data_size = std::mem::size_of::<i128>();
                let data_slice =
                    idl_slice_from_bytes(data, data_offset, data_size)?;
                let data_num = i128::from_le_bytes(data_slice.try_into()?);
                (data_size, json!(data_num))
            },
            ToolboxIdlTypePrimitive::F32 => {
                let data_size = std::mem::size_of::<f32>();
                let data_slice =
                    idl_slice_from_bytes(data, data_offset, data_size)?;
                let data_num = f32::from_le_bytes(data_slice.try_into()?);
                (data_size, json!(data_num))
            },
            ToolboxIdlTypePrimitive::F64 => {
                let data_size = std::mem::size_of::<f64>();
                let data_slice =
                    idl_slice_from_bytes(data, data_offset, data_size)?;
                let data_num = f64::from_le_bytes(data_slice.try_into()?);
                (data_size, json!(data_num))
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

// TODO - serialize deserialize impl separatated for field named/unnamed ?
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
                        .content
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
                        .content
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
