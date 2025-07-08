use std::cmp::max;
use std::collections::HashMap;

use crate::toolbox_idl_encoding as encoding;
use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use convert_case::Boundary;
use convert_case::Case;
use convert_case::Casing;
use serde_json::Map;
use serde_json::Value;
use solana_sdk::hash::Hasher;

use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;

pub(crate) fn idl_object_get_key_as_array<'a>(
    object: &'a Map<String, Value>,
    key: &str,
) -> Option<&'a Vec<Value>> {
    object.get(key).and_then(|value| value.as_array())
}

pub(crate) fn idl_object_get_key_as_object<'a>(
    object: &'a Map<String, Value>,
    key: &str,
) -> Option<&'a Map<String, Value>> {
    object.get(key).and_then(|value| value.as_object())
}

pub(crate) fn idl_object_get_key_as_str<'a>(
    object: &'a Map<String, Value>,
    key: &str,
) -> Option<&'a str> {
    object.get(key).and_then(|value| value.as_str())
}

pub(crate) fn idl_object_get_key_as_u64(
    object: &Map<String, Value>,
    key: &str,
) -> Option<u64> {
    object.get(key).and_then(|value| value.as_u64())
}

pub(crate) fn idl_object_get_key_as_bool(
    object: &Map<String, Value>,
    key: &str,
) -> Option<bool> {
    object.get(key).and_then(|value| value.as_bool())
}

pub(crate) fn idl_object_get_key_or_else<'a>(
    object: &'a Map<String, Value>,
    key: &str,
) -> Result<&'a Value> {
    object.get(key).with_context(|| {
        format!(
            "Expected value at key: {}. Found keys: {:?}",
            key,
            object.keys().collect::<Vec<_>>()
        )
    })
}

pub(crate) fn idl_object_get_key_as_array_or_else<'a>(
    object: &'a Map<String, Value>,
    key: &str,
) -> Result<&'a Vec<Value>> {
    idl_object_get_key_or_else(object, key)?
        .as_array()
        .with_context(|| {
            format!(
                "Expected an array at key: {}, got: {:?}",
                key,
                object.get(key)
            )
        })
}

pub(crate) fn idl_object_get_key_as_str_or_else<'a>(
    object: &'a Map<String, Value>,
    key: &str,
) -> Result<&'a str> {
    idl_object_get_key_or_else(object, key)?
        .as_str()
        .with_context(|| {
            format!(
                "Expected a string at key: {}, got: {:?}",
                key,
                object.get(key)
            )
        })
}

pub(crate) fn idl_object_get_key_as_u64_or_else(
    object: &Map<String, Value>,
    key: &str,
) -> Result<u64> {
    idl_object_get_key_or_else(object, key)?
        .as_u64()
        .with_context(|| {
            format!(
                "Expected a string at key: {}, got: {:?}",
                key,
                object.get(key)
            )
        })
}

pub(crate) fn idl_value_as_str_or_object_with_key_as_str_or_else<'a>(
    value: &'a Value,
    key: &str,
) -> Result<&'a str> {
    match value.as_str() {
        Some(name) => Ok(name),
        None => {
            let object = idl_value_as_object_or_else(value)?;
            Ok(idl_object_get_key_as_str_or_else(object, key)?)
        },
    }
}

pub(crate) fn idl_value_as_object_get_key_as_array<'a>(
    value: &'a Value,
    key: &str,
) -> Option<&'a Vec<Value>> {
    value
        .as_object()
        .and_then(|object| object.get(key))
        .and_then(|item| item.as_array())
}

pub(crate) fn idl_value_as_object_get_key_as_str<'a>(
    value: &'a Value,
    key: &str,
) -> Option<&'a str> {
    value
        .as_object()
        .and_then(|object| object.get(key))
        .and_then(|item| item.as_str())
}

pub(crate) fn idl_value_as_object_get_key_as_u64(
    value: &Value,
    key: &str,
) -> Option<u64> {
    value
        .as_object()
        .and_then(|object| object.get(key))
        .and_then(|item| item.as_u64())
}

pub(crate) fn idl_value_as_object_get_key<'a>(
    value: &'a Value,
    key: &str,
) -> Option<&'a Value> {
    value.as_object().and_then(|object| object.get(key))
}

pub(crate) fn idl_value_as_array_or_else(value: &Value) -> Result<&Vec<Value>> {
    value.as_array().context("Expected an array")
}

pub(crate) fn idl_value_as_object_or_else(
    value: &Value,
) -> Result<&Map<String, Value>> {
    value.as_object().context("Expected an object")
}

pub(crate) fn idl_value_as_str_or_else(value: &Value) -> Result<&str> {
    value.as_str().context("Expected an string")
}

pub(crate) fn idl_value_as_u64_or_else(value: &Value) -> Result<u64> {
    value.as_u64().context("Expected an unsigned number")
}

pub(crate) fn idl_value_as_i64_or_else(value: &Value) -> Result<i64> {
    value.as_i64().context("Expected a signed number")
}

pub(crate) fn idl_value_as_f64_or_else(value: &Value) -> Result<f64> {
    value.as_f64().context("Expected a floating number")
}

pub(crate) fn idl_value_as_bool_or_else(value: &Value) -> Result<bool> {
    value.as_bool().context("Expected a boolean")
}

pub(crate) fn idl_value_as_bytes_or_else(value: &Value) -> Result<Vec<u8>> {
    if let Some(value_array) = value.as_array() {
        let mut bytes = vec![];
        for item in value_array {
            bytes.push(u8::try_from(idl_value_as_u64_or_else(item)?)?);
        }
        return Ok(bytes);
    }
    if let Some(value_object) = value.as_object() {
        if let Some(data) = idl_object_get_key_as_str(value_object, "base16") {
            return encoding::sanitize_and_decode_base16(data);
        }
        if let Some(data) = idl_object_get_key_as_str(value_object, "base58") {
            return encoding::sanitize_and_decode_base58(data);
        }
        if let Some(data) = idl_object_get_key_as_str(value_object, "base64") {
            return encoding::sanitize_and_decode_base64(data);
        }
        if let Some(data) = idl_object_get_key_as_str(value_object, "utf8") {
            return Ok(data.as_bytes().to_vec());
        }
        if let Some(data) = idl_object_get_key_as_u64(value_object, "zeroes") {
            return Ok(vec![0; usize::try_from(data)?]);
        }
        // TODO - support 0xff padding ?
        // TODO - support type/value pairs ?
    }
    Err(anyhow!("Could not read bytes, expected an array/object"))
}

pub(crate) fn idl_slice_from_bytes(
    bytes: &[u8],
    offset: usize,
    length: usize,
) -> Result<&[u8]> {
    let end = offset.checked_add(length).with_context(|| {
        format!(
            "Invalid slice length: offset: 0x{:X}, length: {}",
            offset, length,
        )
    })?;
    if bytes.len() < end {
        return Err(anyhow!(
            "Invalid slice read at offset: 0x{:X}, length: {}, from bytes: {}",
            offset,
            length,
            bytes.len(),
        ));
    }
    Ok(&bytes[offset..end])
}

// TODO - are those needed or can those be inlined ?
pub(crate) fn idl_u8_from_bytes_at(bytes: &[u8], offset: usize) -> Result<u8> {
    let size = std::mem::size_of::<u8>();
    let slice = idl_slice_from_bytes(bytes, offset, size)?;
    Ok(u8::from_le_bytes(slice.try_into()?))
}

pub(crate) fn idl_u16_from_bytes_at(
    bytes: &[u8],
    offset: usize,
) -> Result<u16> {
    let size = std::mem::size_of::<u16>();
    let slice = idl_slice_from_bytes(bytes, offset, size)?;
    Ok(u16::from_le_bytes(slice.try_into()?))
}

pub(crate) fn idl_u32_from_bytes_at(
    bytes: &[u8],
    offset: usize,
) -> Result<u32> {
    let size = std::mem::size_of::<u32>();
    let slice = idl_slice_from_bytes(bytes, offset, size)?;
    Ok(u32::from_le_bytes(slice.try_into()?))
}

pub(crate) fn idl_u64_from_bytes_at(
    bytes: &[u8],
    offset: usize,
) -> Result<u64> {
    let size = std::mem::size_of::<u64>();
    let slice = idl_slice_from_bytes(bytes, offset, size)?;
    Ok(u64::from_le_bytes(slice.try_into()?))
}

pub(crate) fn idl_map_get_key_or_else<'a, V: std::fmt::Debug>(
    map: &'a HashMap<String, V>,
    key: &str,
) -> Result<&'a V> {
    map.get(key).with_context(|| {
        format!(
            "Expected value at key: {}. Found keys: {:?}",
            key,
            map.keys().collect::<Vec<_>>()
        )
    })
}

pub(crate) fn idl_convert_to_snake_case(name: &str) -> String {
    if name.contains(|c: char| {
        !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    }) {
        name.without_boundaries(&[Boundary::UPPER_DIGIT])
            .without_boundaries(&[Boundary::LOWER_DIGIT])
            .to_case(Case::Snake)
    } else {
        name.to_string()
    }
}

pub(crate) fn idl_convert_to_camel_case(name: &str) -> String {
    if name.contains(|c: char| !c.is_ascii_alphanumeric()) {
        name.without_boundaries(&[Boundary::UPPER_DIGIT])
            .without_boundaries(&[Boundary::LOWER_DIGIT])
            .to_case(Case::Camel)
    } else {
        name.to_string()
    }
}

pub(crate) fn idl_hash_discriminator_from_string(value: &str) -> Vec<u8> {
    let mut hasher = Hasher::default();
    hasher.hash(value.as_bytes());
    hasher.result().to_bytes()[..8].to_vec()
}

pub(crate) fn idl_alignment_padding_needed(
    offset: usize,
    alignment: usize,
) -> usize {
    let missalignment = offset % alignment;
    if missalignment == 0 {
        return 0;
    }
    alignment - missalignment
}

#[allow(clippy::type_complexity)]
pub(crate) fn idl_fields_infos_aligned<T>(
    prefix_size: usize,
    fields_infos: Vec<(usize, usize, T, ToolboxIdlTypeFull)>,
) -> Result<(usize, usize, Vec<(T, ToolboxIdlTypeFull)>)> {
    let mut alignment = prefix_size;
    let mut size = prefix_size;
    let last_field_index = fields_infos.len().saturating_sub(1);
    let mut fields_infos_padded = vec![];
    for (field_index, field_info) in fields_infos.into_iter().enumerate() {
        let (field_alignment, field_size, field_meta, field_type) = field_info;
        alignment = max(alignment, field_alignment);
        let padding_before =
            idl_alignment_padding_needed(size, field_alignment);
        size += padding_before + field_size;
        let padding_after = if field_index == last_field_index {
            idl_alignment_padding_needed(size, alignment)
        } else {
            0
        };
        size += padding_after;
        if padding_before == 0 && padding_after == 0 {
            fields_infos_padded.push((field_meta, field_type));
        } else {
            fields_infos_padded.push((
                field_meta,
                ToolboxIdlTypeFull::Padded {
                    before: padding_before,
                    min_size: field_size,
                    after: padding_after,
                    content: Box::new(field_type),
                },
            ));
        }
    }
    Ok((alignment, size, fields_infos_padded))
}
