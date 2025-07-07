use std::cmp::max;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use serde_json::json;
use serde_json::Map;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullEnumVariant;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;
use crate::toolbox_idl_type_prefix::ToolboxIdlTypePrefix;
use crate::toolbox_idl_type_primitive::ToolboxIdlTypePrimitive;
use crate::toolbox_idl_utils::idl_slice_from_bytes;
use crate::toolbox_idl_utils::idl_u16_from_bytes_at;
use crate::toolbox_idl_utils::idl_u32_from_bytes_at;
use crate::toolbox_idl_utils::idl_u64_from_bytes_at;
use crate::toolbox_idl_utils::idl_u8_from_bytes_at;

impl ToolboxIdlTypeFull {
    pub fn try_deserialize(
        &self,
        data: &[u8],
        data_offset: usize,
        // TODO (FAR) - support deserializing bytes into custom stuff like base64 ?
    ) -> Result<(usize, Value)> {
        match self {
            ToolboxIdlTypeFull::Typedef { name, content, .. } => {
                ToolboxIdlTypeFull::try_deserialize(content, data, data_offset)
                    .with_context(|| {
                        format!(
                            "Deserialize Typedef, name: {} (offset: {})",
                            name, data_offset
                        )
                    })
            },
            ToolboxIdlTypeFull::Option {
                prefix, content, ..
            } => ToolboxIdlTypeFull::try_deserialize_option(
                prefix,
                content,
                data,
                data_offset,
            )
            .with_context(|| {
                format!(
                    "Deserialize Option, prefix: {} (offset: {})",
                    prefix.as_str(),
                    data_offset
                )
            }),
            ToolboxIdlTypeFull::Vec { prefix, items, .. } => {
                ToolboxIdlTypeFull::try_deserialize_vec(
                    prefix,
                    items,
                    data,
                    data_offset,
                )
            }
            .with_context(|| {
                format!(
                    "Deserialize Vec, prefix: {} (offset: {})",
                    prefix.as_str(),
                    data_offset
                )
            }),
            ToolboxIdlTypeFull::Array { items, length } => {
                ToolboxIdlTypeFull::try_deserialize_array(
                    items,
                    length,
                    data,
                    data_offset,
                )
            }
            .with_context(|| {
                format!(
                    "Deserialize Array, length: {} (offset: {})",
                    length, data_offset
                )
            }),
            ToolboxIdlTypeFull::String { prefix, .. } => {
                ToolboxIdlTypeFull::try_deserialize_string(
                    prefix,
                    data,
                    data_offset,
                )
            }
            .with_context(|| {
                format!("Deserialize String (offset: {})", data_offset)
            }),
            ToolboxIdlTypeFull::Struct { fields, .. } => {
                ToolboxIdlTypeFull::try_deserialize_struct(
                    fields,
                    data,
                    data_offset,
                )
            }
            .with_context(|| {
                format!("Deserialize Struct (offset: {})", data_offset)
            }),
            ToolboxIdlTypeFull::Enum {
                prefix, variants, ..
            } => ToolboxIdlTypeFull::try_deserialize_enum(
                prefix,
                variants,
                data,
                data_offset,
            )
            .with_context(|| {
                format!(
                    "Deserialize Enum, prefix: {} (offset: {})",
                    prefix.as_str(),
                    data_offset
                )
            }),
            ToolboxIdlTypeFull::Padded {
                before,
                min_size,
                after,
                content,
            } => ToolboxIdlTypeFull::try_deserialize_padded(
                before,
                min_size,
                after,
                content,
                data,
                data_offset,
            )
            .with_context(|| {
                format!(
                    "Deserialize Padded, spaces: {}/{}/{} (offset: {})",
                    before, min_size, after, data_offset
                )
            }),
            ToolboxIdlTypeFull::Const { literal } => Err(anyhow!(
                "Deserialize Const: Can't use a const literal directly: {}",
                literal
            )),
            ToolboxIdlTypeFull::Primitive { primitive } => primitive
                .try_deserialize(data, data_offset)
                .with_context(|| {
                    format!(
                        "Deserialize Primitive: {} (offset: {})",
                        primitive.as_str(),
                        data_offset
                    )
                }),
        }
    }

    fn try_deserialize_option(
        option_prefix: &ToolboxIdlTypePrefix,
        option_content: &ToolboxIdlTypeFull,
        data: &[u8],
        data_offset: usize,
    ) -> Result<(usize, Value)> {
        let (mut data_size, data_prefix) =
            option_prefix.try_deserialize(data, data_offset)?;
        if (data_prefix & 1) == 0 {
            return Ok((data_size, Value::Null));
        }
        let data_content_offset = data_offset + data_size;
        let (data_content_size, data_content) = option_content
            .try_deserialize(data, data_content_offset)
            .with_context(|| {
                format!(
                    "Deserialize Option Content (offset: {})",
                    data_content_offset
                )
            })?;
        data_size += data_content_size;
        Ok((data_size, data_content))
    }

    fn try_deserialize_vec(
        vec_prefix: &ToolboxIdlTypePrefix,
        vec_items: &ToolboxIdlTypeFull,
        data: &[u8],
        data_offset: usize,
    ) -> Result<(usize, Value)> {
        let (mut data_size, data_prefix) =
            vec_prefix.try_deserialize(data, data_offset)?;
        let mut data_items = vec![];
        for index in 0..data_prefix {
            let data_item_offset = data_offset + data_size;
            let (data_item_size, data_item) = vec_items
                .try_deserialize(data, data_item_offset)
                .with_context(|| {
                    format!(
                        "Deserialize Vec Item: {} (offset: {})",
                        index, data_item_offset
                    )
                })?;
            data_size += data_item_size;
            data_items.push(data_item);
        }
        Ok((data_size, json!(data_items)))
    }

    fn try_deserialize_array(
        array_items: &ToolboxIdlTypeFull,
        array_length: &usize,
        data: &[u8],
        data_offset: usize,
    ) -> Result<(usize, Value)> {
        let mut data_size = 0;
        let mut data_items = vec![];
        for index in 0..*array_length {
            let data_item_offset = data_offset + data_size;
            let (data_item_size, data_item) = array_items
                .try_deserialize(data, data_item_offset)
                .with_context(|| {
                    format!(
                        "Deserialize Array Item: {} (offset: {})",
                        index, data_item_offset
                    )
                })?;
            data_size += data_item_size;
            data_items.push(data_item);
        }
        Ok((data_size, json!(data_items)))
    }

    fn try_deserialize_string(
        string_prefix: &ToolboxIdlTypePrefix,
        data: &[u8],
        data_offset: usize,
    ) -> Result<(usize, Value)> {
        let (mut data_size, data_prefix) =
            string_prefix.try_deserialize(data, data_offset)?;
        let data_chars_offset = data_offset + data_size;
        let data_bytes = idl_slice_from_bytes(
            data,
            data_chars_offset,
            usize::try_from(data_prefix)?,
        )?;
        data_size += data_bytes.len();
        Ok((data_size, json!(String::from_utf8(data_bytes.to_vec())?)))
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
        let mut enum_mask = 0;
        for enum_variant in enum_variants {
            enum_mask |= enum_variant.code;
        }
        let (mut data_size, data_prefix) =
            enum_prefix.try_deserialize(data, data_offset)?;
        let data_variant_offset = data_offset + data_size;
        for enum_variant in enum_variants {
            if enum_variant.code == (data_prefix & enum_mask) {
                let (data_variant_size, data_variant) =
                    ToolboxIdlTypeFull::try_deserialize_enum_variant(
                        enum_variant,
                        data,
                        data_variant_offset,
                    )?;
                data_size += data_variant_size;
                return Ok((data_size, data_variant));
            }
        }
        Err(anyhow!(
            "Deserialize Enum Unknown Code: {} (offset: {}), known variants: {}",
            data_prefix,
            data_offset,
            enum_variants
                .iter()
                .map(|enum_variant| format!(
                    "{}={}",
                    enum_variant.name, enum_variant.code
                ))
                .collect::<Vec<_>>()
                .join(", "),
        ))
    }

    fn try_deserialize_enum_variant(
        enum_variant: &ToolboxIdlTypeFullEnumVariant,
        data: &[u8],
        data_offset: usize,
    ) -> Result<(usize, Value)> {
        let (data_fields_size, data_fields) = enum_variant
            .fields
            .try_deserialize(data, data_offset)
            .with_context(|| {
                format!(
                    "Deserialize Enum Variant Name: {} (offset: {})",
                    enum_variant.name, data_offset
                )
            })?;
        if data_fields.is_null() {
            return Ok((data_fields_size, json!(enum_variant.name)));
        }
        Ok((
            data_fields_size,
            json!({
                enum_variant.name.to_string(): data_fields
            }),
        ))
    }

    fn try_deserialize_padded(
        padded_before: &usize,
        padded_min_size: &usize,
        padded_after: &usize,
        padded_content: &ToolboxIdlTypeFull,
        data: &[u8],
        data_offset: usize,
    ) -> Result<(usize, Value)> {
        let mut data_size = *padded_before;
        let data_content_offset = data_offset + data_size;
        let (data_content_size, data_content) = padded_content
            .try_deserialize(data, data_content_offset)
            .with_context(|| {
                format!(
                    "Deserialize Padded Content (offset: {})",
                    data_content_offset
                )
            })?;
        data_size += max(data_content_size, *padded_min_size);
        data_size += *padded_after;
        Ok((data_size, data_content))
    }
}

impl ToolboxIdlTypeFullFields {
    pub fn try_deserialize(
        &self,
        data: &[u8],
        data_offset: usize,
    ) -> Result<(usize, Value)> {
        if self.is_empty() {
            return Ok((0, json!(null)));
        }
        Ok(match self {
            ToolboxIdlTypeFullFields::Named(fields) => {
                let mut data_size = 0;
                let mut data_fields = Map::new();
                for field in fields {
                    let data_field_offset = data_offset + data_size;
                    let (data_field_size, data_field) = field
                        .content
                        .try_deserialize(data, data_field_offset)
                        .with_context(|| {
                            format!(
                                "Deserialize Field: {} (offset: {})",
                                field.name, data_field_offset
                            )
                        })?;
                    data_size += data_field_size;
                    data_fields.insert(field.name.to_string(), data_field);
                }
                (data_size, json!(data_fields))
            },
            ToolboxIdlTypeFullFields::Unnamed(fields) => {
                let mut data_size = 0;
                let mut data_fields = vec![json!(null); fields.len()];
                for field in fields {
                    let data_field_offset = data_offset + data_size;
                    let (data_field_size, data_field) = field
                        .content
                        .try_deserialize(data, data_field_offset)
                        .with_context(|| {
                            format!(
                                "Deserialize Field: {} (offset: {})",
                                field.position, data_field_offset
                            )
                        })?;
                    data_size += data_field_size;
                    data_fields[field.position] = data_field;
                }
                (data_size, json!(data_fields))
            },
        })
    }
}

impl ToolboxIdlTypePrefix {
    pub fn try_deserialize(
        &self,
        data: &[u8],
        data_offset: usize,
    ) -> Result<(usize, u64)> {
        Ok((
            self.to_size(),
            match self {
                ToolboxIdlTypePrefix::U8 => {
                    idl_u8_from_bytes_at(data, data_offset)?.into()
                },
                ToolboxIdlTypePrefix::U16 => {
                    idl_u16_from_bytes_at(data, data_offset)?.into()
                },
                ToolboxIdlTypePrefix::U32 => {
                    idl_u32_from_bytes_at(data, data_offset)?.into()
                },
                ToolboxIdlTypePrefix::U64 => {
                    idl_u64_from_bytes_at(data, data_offset)?
                },
                ToolboxIdlTypePrefix::U128 => {
                    idl_u64_from_bytes_at(data, data_offset)?
                },
            },
        ))
    }
}

impl ToolboxIdlTypePrimitive {
    fn try_deserialize(
        self: &ToolboxIdlTypePrimitive,
        data: &[u8],
        data_offset: usize,
    ) -> Result<(usize, Value)> {
        Ok(match self {
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
            ToolboxIdlTypePrimitive::Bool => {
                let data_size = 1;
                let data_slice =
                    idl_slice_from_bytes(data, data_offset, data_size)?;
                let data_flag = data_slice[0] != 0;
                (data_size, json!(data_flag))
            },
            ToolboxIdlTypePrimitive::Pubkey => {
                let data_size = std::mem::size_of::<Pubkey>();
                let data_slice =
                    idl_slice_from_bytes(data, data_offset, data_size)?;
                let data_pubkey =
                    Pubkey::new_from_array(data_slice.try_into()?);
                (data_size, json!(data_pubkey.to_string()))
            },
        })
    }
}
