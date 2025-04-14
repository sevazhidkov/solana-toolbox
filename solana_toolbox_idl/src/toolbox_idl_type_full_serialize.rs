use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;
use crate::toolbox_idl_type_primitive::ToolboxIdlTypePrimitive;
use crate::toolbox_idl_utils::idl_as_array_or_else;
use crate::toolbox_idl_utils::idl_as_bool_or_else;
use crate::toolbox_idl_utils::idl_as_bytes_or_else;
use crate::toolbox_idl_utils::idl_as_f64_or_else;
use crate::toolbox_idl_utils::idl_as_i64_or_else;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_as_str_or_else;
use crate::toolbox_idl_utils::idl_as_u64_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_object_get_key_as_u64;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;
use crate::toolbox_idl_utils::idl_prefix_write;

impl ToolboxIdlTypeFull {
    pub fn try_serialize(
        &self,
        value: &Value,
        data: &mut Vec<u8>,
        // TODO (FAR) - Config object for pubkey hashmap and prefixes and existing
        deserializable: bool,
    ) -> Result<()> {
        match self {
            ToolboxIdlTypeFull::Option {
                prefix_bytes,
                content,
            } => ToolboxIdlTypeFull::try_serialize_option(
                prefix_bytes,
                content,
                value,
                data,
                deserializable,
            ),
            ToolboxIdlTypeFull::Vec {
                prefix_bytes,
                items,
            } => ToolboxIdlTypeFull::try_serialize_vec(
                prefix_bytes,
                items,
                value,
                data,
                deserializable,
            ),
            ToolboxIdlTypeFull::Array { items, length } => {
                ToolboxIdlTypeFull::try_serialize_array(
                    items,
                    length,
                    value,
                    data,
                    deserializable,
                )
            },
            ToolboxIdlTypeFull::Struct { fields } => {
                ToolboxIdlTypeFull::try_serialize_struct(
                    fields,
                    value,
                    data,
                    deserializable,
                )
            },
            ToolboxIdlTypeFull::Enum {
                prefix_bytes,
                variants,
            } => ToolboxIdlTypeFull::try_serialize_enum(
                prefix_bytes,
                variants,
                value,
                data,
                deserializable,
            ),
            ToolboxIdlTypeFull::Padded {
                size_bytes,
                content,
            } => ToolboxIdlTypeFull::try_serialize_padded(
                size_bytes,
                content,
                value,
                data,
                deserializable,
            ),
            ToolboxIdlTypeFull::Const { literal } => {
                Err(anyhow!("Can't use a const literal directly: {}", literal))
            },
            ToolboxIdlTypeFull::Primitive { primitive } => {
                ToolboxIdlTypeFull::try_serialize_primitive(
                    primitive,
                    value,
                    data,
                    deserializable,
                )
            },
        }
    }

    fn try_serialize_option(
        option_prefix_bytes: &u8,
        option_content: &ToolboxIdlTypeFull,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
    ) -> Result<()> {
        if value.is_null() {
            idl_prefix_write(option_prefix_bytes, 0, data)?;
            Ok(())
        } else {
            idl_prefix_write(option_prefix_bytes, 1, data)?;
            option_content.try_serialize(value, data, deserializable)
        }
    }

    fn try_serialize_vec(
        vec_prefix_bytes: &u8,
        vec_items: &ToolboxIdlTypeFull,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
    ) -> Result<()> {
        if vec_items.is_primitive(&ToolboxIdlTypePrimitive::U8) {
            let bytes = try_read_value_to_bytes(value)?;
            if deserializable {
                idl_prefix_write(vec_prefix_bytes, bytes.len(), data)?;
            }
            data.extend_from_slice(&bytes);
            return Ok(());
        }
        let values = idl_as_array_or_else(value)?;
        if deserializable {
            idl_prefix_write(vec_prefix_bytes, values.len(), data)?;
        }
        for (index, value_item) in values.iter().enumerate() {
            vec_items
                .try_serialize(value_item, data, deserializable)
                .with_context(|| format!("Serialize Vec Item: {}", index))?;
        }
        Ok(())
    }

    fn try_serialize_array(
        array_items: &ToolboxIdlTypeFull,
        array_length: &u64,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
    ) -> Result<()> {
        let array_length = usize::try_from(*array_length)?;
        if array_items.is_primitive(&ToolboxIdlTypePrimitive::U8) {
            let bytes = try_read_value_to_bytes(value)?;
            if bytes.len() != array_length {
                return Err(anyhow!(
                    "value byte array is not the correct size: expected {} bytes, found {} bytes",
                    array_length,
                    bytes.len()
                ));
            }
            data.extend_from_slice(&bytes);
            return Ok(());
        }
        let values = idl_as_array_or_else(value)?;
        if values.len() != array_length {
            return Err(anyhow!(
                "value array is not the correct size: expected {} items, found {} items",
                array_length,
                values.len()
            ));
        }
        for (index, value_item) in values.iter().enumerate() {
            array_items
                .try_serialize(value_item, data, deserializable)
                .with_context(|| format!("Serialize Array Item: {}", index))?;
        }
        Ok(())
    }

    fn try_serialize_struct(
        struct_fields: &ToolboxIdlTypeFullFields,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
    ) -> Result<()> {
        struct_fields.try_serialize(value, data, deserializable)
    }

    fn try_serialize_enum(
        enum_prefix_bytes: &u8,
        enum_variants: &[(String, ToolboxIdlTypeFullFields)],
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
    ) -> Result<()> {
        // TODO (FAR) - support non-sequential enums ?
        if let Some(value_string) = value.as_str() {
            for (enum_code, enum_variant) in enum_variants.iter().enumerate() {
                let (enum_variant_name, enum_variant_fields) = enum_variant;
                if enum_variant_name == value_string {
                    idl_prefix_write(enum_prefix_bytes, enum_code, data)?;
                    return enum_variant_fields
                        .try_serialize(&Value::Null, data, deserializable)
                        .with_context(|| {
                            format!(
                                "Serialize Enum Variant: {}",
                                enum_variant_name
                            )
                        });
                }
            }
            return Err(anyhow!(
                "Could not guess enum string: {}",
                value_string
            ));
        }
        if let Some(value_object) = value.as_object() {
            for (enum_code, enum_variant) in enum_variants.iter().enumerate() {
                let (enum_variant_name, enum_variant_fields) = enum_variant;
                if let Some(enum_value) = value_object.get(enum_variant_name) {
                    idl_prefix_write(enum_prefix_bytes, enum_code, data)?;
                    return enum_variant_fields
                        .try_serialize(enum_value, data, deserializable)
                        .with_context(|| {
                            format!(
                                "Serialize Enum Variant: {}",
                                enum_variant_name
                            )
                        });
                }
            }
            return Err(anyhow!("Could not guess enum object key"));
        }
        Err(anyhow!("Expected enum value to be: object or string"))
    }

    fn try_serialize_padded(
        padded_size_bytes: &u64,
        padded_content: &ToolboxIdlTypeFull,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
    ) -> Result<()> {
        let padded_size_bytes = usize::try_from(*padded_size_bytes)?;
        let data_len_enforced = data.len() + padded_size_bytes;
        padded_content.try_serialize(value, data, deserializable)?;
        while data.len() < data_len_enforced {
            data.push(0);
        }
        Ok(())
    }

    fn try_serialize_primitive(
        primitive: &ToolboxIdlTypePrimitive,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
    ) -> Result<()> {
        match primitive {
            ToolboxIdlTypePrimitive::U8 => {
                let value_integer = idl_as_u64_or_else(value)?;
                let value_typed = u8::try_from(value_integer)?;
                data.extend_from_slice(&value_typed.to_le_bytes());
            },
            ToolboxIdlTypePrimitive::U16 => {
                let value_integer = idl_as_u64_or_else(value)?;
                let value_typed = u16::try_from(value_integer)?;
                data.extend_from_slice(&value_typed.to_le_bytes());
            },
            ToolboxIdlTypePrimitive::U32 => {
                let value_integer = idl_as_u64_or_else(value)?;
                let value_typed = u32::try_from(value_integer)?;
                data.extend_from_slice(&value_typed.to_le_bytes());
            },
            ToolboxIdlTypePrimitive::U64 => {
                let value_integer = idl_as_u64_or_else(value)?;
                data.extend_from_slice(&value_integer.to_le_bytes());
            },
            ToolboxIdlTypePrimitive::U128 => {
                let value_integer = u128::from(idl_as_u64_or_else(value)?);
                data.extend_from_slice(&value_integer.to_le_bytes());
            },
            ToolboxIdlTypePrimitive::I8 => {
                let value_integer = idl_as_i64_or_else(value)?;
                let value_typed = i8::try_from(value_integer)?;
                data.extend_from_slice(&value_typed.to_le_bytes());
            },
            ToolboxIdlTypePrimitive::I16 => {
                let value_integer = idl_as_i64_or_else(value)?;
                let value_typed = i16::try_from(value_integer)?;
                data.extend_from_slice(&value_typed.to_le_bytes());
            },
            ToolboxIdlTypePrimitive::I32 => {
                let value_integer = idl_as_i64_or_else(value)?;
                let value_typed = i32::try_from(value_integer)?;
                data.extend_from_slice(&value_typed.to_le_bytes());
            },
            ToolboxIdlTypePrimitive::I64 => {
                let value_integer = idl_as_i64_or_else(value)?;
                data.extend_from_slice(&value_integer.to_le_bytes());
            },
            ToolboxIdlTypePrimitive::I128 => {
                let value_integer = i128::from(idl_as_i64_or_else(value)?);
                data.extend_from_slice(&value_integer.to_le_bytes());
            },
            ToolboxIdlTypePrimitive::F32 => {
                let value_floating = idl_as_f64_or_else(value)? as f32;
                data.extend_from_slice(&value_floating.to_le_bytes());
            },
            ToolboxIdlTypePrimitive::F64 => {
                let value_floating = idl_as_f64_or_else(value)?;
                data.extend_from_slice(&value_floating.to_le_bytes());
            },
            ToolboxIdlTypePrimitive::Boolean => {
                data.push(if idl_as_bool_or_else(value)? { 1 } else { 0 });
            },
            ToolboxIdlTypePrimitive::String => {
                let value_str = idl_as_str_or_else(value)?;
                if deserializable {
                    let value_length = u32::try_from(value_str.len())?;
                    data.extend_from_slice(&value_length.to_le_bytes());
                }
                data.extend_from_slice(value_str.as_bytes());
            },
            ToolboxIdlTypePrimitive::PublicKey => {
                let value_str = idl_as_str_or_else(value)?;
                let value_pubkey = Pubkey::from_str(value_str)?;
                data.extend_from_slice(&value_pubkey.to_bytes());
            },
        };
        Ok(())
    }
}

impl ToolboxIdlTypeFullFields {
    pub fn try_serialize(
        &self,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
    ) -> Result<()> {
        match self {
            ToolboxIdlTypeFullFields::Named(fields) => {
                let value = idl_as_object_or_else(value)?;
                for (field_name, field_type) in fields {
                    let value_field =
                        idl_object_get_key_or_else(value, field_name)?;
                    field_type
                        .try_serialize(value_field, data, deserializable)
                        .with_context(|| {
                            format!("Serialize Field: {}", field_name)
                        })?;
                }
            },
            ToolboxIdlTypeFullFields::Unnamed(fields) => {
                let values = idl_as_array_or_else(value)?;
                if values.len() != fields.len() {
                    return Err(anyhow!("Wrong number of unamed fields, expected: {}, found: {}", fields.len(), values.len()));
                }
                for (index, field) in fields.iter().enumerate() {
                    field
                        .try_serialize(&values[index], data, deserializable)
                        .with_context(|| format!("Serialize Field: {}", index))?;
                }
            },
            ToolboxIdlTypeFullFields::None => {},
        }
        Ok(())
    }
}

fn try_read_value_to_bytes(value: &Value) -> Result<Vec<u8>> {
    if let Some(value_array) = value.as_array() {
        return idl_as_bytes_or_else(value_array);
    }
    if let Some(value_object) = value.as_object() {
        if let Some(data) = idl_object_get_key_as_str(value_object, "hex") {
            return try_read_hex_to_bytes(data);
        }
        if let Some(data) = idl_object_get_key_as_str(value_object, "base58") {
            return ToolboxEndpoint::sanitize_and_decode_base58(data);
        }
        if let Some(data) = idl_object_get_key_as_str(value_object, "base64") {
            return ToolboxEndpoint::sanitize_and_decode_base64(data);
        }
        if let Some(data) = idl_object_get_key_as_str(value_object, "utf8") {
            return Ok(data.as_bytes().to_vec());
        }
        if let Some(data) = idl_object_get_key_as_u64(value_object, "zeroes") {
            return Ok(vec![0; usize::try_from(data)?]);
        }
    }
    Err(anyhow!("Could not read bytes, expected an array/object"))
}

fn try_read_hex_to_bytes(data: &str) -> Result<Vec<u8>> {
    let hex = data.replace(|c| !char::is_ascii_alphanumeric(&c), "");
    let mut bytes = vec![];
    for byte in 0..(hex.len() / 2) {
        let byte_idx = byte * 2;
        let byte_hex = &hex[byte_idx..byte_idx + 2];
        bytes.push(u8::from_str_radix(byte_hex, 16)?);
    }
    Ok(bytes)
}
