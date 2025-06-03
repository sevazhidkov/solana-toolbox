use std::cmp::max;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullEnumVariant;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;
use crate::toolbox_idl_type_prefix::ToolboxIdlTypePrefix;
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

impl ToolboxIdlTypeFull {
    pub fn try_serialize(
        &self,
        value: &Value,
        data: &mut Vec<u8>,
        // TODO (FAR) - Config object for allowing numbers as string
        // TODO (FAR) - Config object for pubkey hashmap and prefixes and existing
        // TODO (FAR) - Config object for custom serializers ?
        deserializable: bool,
    ) -> Result<()> {
        match self {
            ToolboxIdlTypeFull::Typedef { name, content, .. } => {
                ToolboxIdlTypeFull::try_serialize(
                    content,
                    value,
                    data,
                    deserializable,
                )
                .with_context(|| format!("Serialize Typedef, name: {}", name))
            },
            ToolboxIdlTypeFull::Option {
                prefix, content, ..
            } => ToolboxIdlTypeFull::try_serialize_option(
                prefix,
                content,
                value,
                data,
                deserializable,
            ),
            ToolboxIdlTypeFull::Vec { prefix, items, .. } => {
                ToolboxIdlTypeFull::try_serialize_vec(
                    prefix,
                    items,
                    value,
                    data,
                    deserializable,
                )
            },
            ToolboxIdlTypeFull::Array { items, length } => {
                ToolboxIdlTypeFull::try_serialize_array(
                    items,
                    length,
                    value,
                    data,
                    deserializable,
                )
            },
            ToolboxIdlTypeFull::String { prefix, .. } => {
                ToolboxIdlTypeFull::try_serialize_string(
                    prefix,
                    value,
                    data,
                    deserializable,
                )
            },
            ToolboxIdlTypeFull::Struct { fields, .. } => {
                ToolboxIdlTypeFull::try_serialize_struct(
                    fields,
                    value,
                    data,
                    deserializable,
                )
            },
            ToolboxIdlTypeFull::Enum {
                prefix, variants, ..
            } => ToolboxIdlTypeFull::try_serialize_enum(
                prefix,
                variants,
                value,
                data,
                deserializable,
            ),
            ToolboxIdlTypeFull::Padded {
                before,
                min_size,
                after,
                content,
            } => ToolboxIdlTypeFull::try_serialize_padded(
                before,
                min_size,
                after,
                content,
                value,
                data,
                deserializable,
            ),
            ToolboxIdlTypeFull::Const { literal } => {
                Err(anyhow!("Can't use a const literal directly: {}", literal))
            },
            ToolboxIdlTypeFull::Primitive { primitive } => {
                primitive.try_serialize(value, data)
            },
        }
    }

    fn try_serialize_option(
        option_prefix: &ToolboxIdlTypePrefix,
        option_content: &ToolboxIdlTypeFull,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
    ) -> Result<()> {
        if value.is_null() {
            option_prefix.try_serialize(0, data)?;
        } else {
            option_prefix.try_serialize(1, data)?;
            option_content.try_serialize(value, data, deserializable)?;
        }
        Ok(())
    }

    fn try_serialize_vec(
        vec_prefix: &ToolboxIdlTypePrefix,
        vec_items: &ToolboxIdlTypeFull,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
    ) -> Result<()> {
        if vec_items.is_primitive(&ToolboxIdlTypePrimitive::U8) {
            let bytes = try_read_value_to_bytes(value)?;
            if deserializable {
                vec_prefix.try_serialize(u64::try_from(bytes.len())?, data)?;
            }
            data.extend_from_slice(&bytes);
            return Ok(());
        }
        let values = idl_as_array_or_else(value)?;
        if deserializable {
            vec_prefix.try_serialize(u64::try_from(values.len())?, data)?;
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
        array_length: &usize,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
    ) -> Result<()> {
        if array_items.is_primitive(&ToolboxIdlTypePrimitive::U8) {
            let bytes = try_read_value_to_bytes(value)?;
            if bytes.len() != *array_length {
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
        if values.len() != *array_length {
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

    fn try_serialize_string(
        string_prefix: &ToolboxIdlTypePrefix,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
    ) -> Result<()> {
        let value_str = idl_as_str_or_else(value)?;
        if deserializable {
            string_prefix
                .try_serialize(u64::try_from(value_str.len())?, data)?;
        }
        data.extend_from_slice(value_str.as_bytes());
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
        enum_prefix: &ToolboxIdlTypePrefix,
        enum_variants: &[ToolboxIdlTypeFullEnumVariant],
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
    ) -> Result<()> {
        if let Some(value_number) = value.as_u64() {
            for enum_variant in enum_variants {
                if enum_variant.code == value_number {
                    enum_prefix.try_serialize(enum_variant.code, data)?;
                    return ToolboxIdlTypeFull::try_serialize_enum_variant(
                        enum_variant,
                        &Value::Null,
                        data,
                        deserializable,
                    );
                }
            }
            return Err(anyhow!(
                "Could not find enum variant with code: {}",
                value_number
            ));
        }
        if let Some(value_string) = value.as_str() {
            for enum_variant in enum_variants {
                if enum_variant.name == value_string {
                    enum_prefix.try_serialize(enum_variant.code, data)?;
                    return ToolboxIdlTypeFull::try_serialize_enum_variant(
                        enum_variant,
                        &Value::Null,
                        data,
                        deserializable,
                    );
                }
            }
            return Err(anyhow!(
                "Could not find enum variant with name: {}",
                value_string
            ));
        }
        if let Some(value_object) = value.as_object() {
            for enum_variant in enum_variants {
                if let Some(enum_value) = value_object.get(&enum_variant.name) {
                    enum_prefix.try_serialize(enum_variant.code, data)?;
                    return ToolboxIdlTypeFull::try_serialize_enum_variant(
                        enum_variant,
                        enum_value,
                        data,
                        deserializable,
                    );
                }
            }
            return Err(anyhow!("Could not guess enum from object keys"));
        }
        Err(anyhow!(
            "Expected enum value to be: number/string or object"
        ))
    }

    fn try_serialize_enum_variant(
        enum_variant: &ToolboxIdlTypeFullEnumVariant,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
    ) -> Result<()> {
        enum_variant
            .fields
            .try_serialize(value, data, deserializable)
            .with_context(|| {
                format!("Serialize Enum Variant: {}", enum_variant.name)
            })
    }

    fn try_serialize_padded(
        padded_before: &usize,
        padded_min_size: &usize,
        padded_after: &usize,
        padded_content: &ToolboxIdlTypeFull,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
    ) -> Result<()> {
        let data_offset_before = data.len() + *padded_before;
        while data.len() < data_offset_before {
            data.push(0);
        }
        padded_content.try_serialize(value, data, deserializable)?;
        let data_content_size = data.len() - data_offset_before;
        let data_offset_after = data_offset_before
            + max(*padded_min_size, data_content_size)
            + *padded_after;
        while data.len() < data_offset_after {
            data.push(0);
        }
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
        if self.len() == 0 {
            return Ok(());
        }
        match self {
            ToolboxIdlTypeFullFields::Named(fields) => {
                let value = idl_as_object_or_else(value)?;
                for field in fields {
                    let value_field =
                        idl_object_get_key_or_else(value, &field.name)?;
                    field
                        .content
                        .try_serialize(value_field, data, deserializable)
                        .with_context(|| {
                            format!("Serialize Field: {}", field.name)
                        })?;
                }
            },
            ToolboxIdlTypeFullFields::Unnamed(fields) => {
                let values = idl_as_array_or_else(value)?;
                if values.len() != fields.len() {
                    return Err(anyhow!("Wrong number of unnamed fields, expected: {}, found: {}", fields.len(), values.len()));
                }
                for field in fields {
                    let value_field = &values[field.position];
                    field
                        .content
                        .try_serialize(value_field, data, deserializable)
                        .with_context(|| {
                            format!("Serialize Field: {}", field.position)
                        })?;
                }
            },
        }
        Ok(())
    }
}

impl ToolboxIdlTypePrefix {
    pub fn try_serialize(&self, value: u64, data: &mut Vec<u8>) -> Result<()> {
        match self {
            ToolboxIdlTypePrefix::U8 => {
                data.extend_from_slice(&u8::try_from(value)?.to_le_bytes())
            },
            ToolboxIdlTypePrefix::U16 => {
                data.extend_from_slice(&u16::try_from(value)?.to_le_bytes())
            },
            ToolboxIdlTypePrefix::U32 => {
                data.extend_from_slice(&u32::try_from(value)?.to_le_bytes())
            },
            ToolboxIdlTypePrefix::U64 => {
                data.extend_from_slice(&value.to_le_bytes())
            },
            ToolboxIdlTypePrefix::U128 => {
                data.extend_from_slice(&u128::from(value).to_le_bytes())
            },
        }
        Ok(())
    }
}

impl ToolboxIdlTypePrimitive {
    pub fn try_serialize(
        self: &ToolboxIdlTypePrimitive,
        value: &Value,
        data: &mut Vec<u8>,
    ) -> Result<()> {
        match self {
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
            ToolboxIdlTypePrimitive::Bool => {
                let value_boolean = idl_as_bool_or_else(value)?;
                data.push(if value_boolean { 1 } else { 0 });
            },
            ToolboxIdlTypePrimitive::Pubkey => {
                let value_str = idl_as_str_or_else(value)?;
                let value_pubkey = Pubkey::from_str(value_str)?;
                data.extend_from_slice(&value_pubkey.to_bytes());
            },
        };
        Ok(())
    }
}

fn try_read_value_to_bytes(value: &Value) -> Result<Vec<u8>> {
    if let Some(value_array) = value.as_array() {
        return idl_as_bytes_or_else(value_array);
    }
    if let Some(value_object) = value.as_object() {
        if let Some(data) = idl_object_get_key_as_str(value_object, "base16") {
            return ToolboxEndpoint::sanitize_and_decode_base16(data);
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
