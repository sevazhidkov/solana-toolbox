use std::collections::HashMap;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use convert_case::Boundary;
use convert_case::Case;
use convert_case::Casing;
use serde_json::Map;
use serde_json::Value;
use sha2::Digest;
use sha2::Sha256;
use solana_sdk::pubkey::Pubkey;

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
    object.get(key).ok_or_else(|| {
        anyhow!(
            "missing value at key: {}, available keys: {:?}",
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
        .ok_or_else(|| {
            anyhow!(
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
        .ok_or_else(|| {
            anyhow!(
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
        .ok_or_else(|| {
            anyhow!(
                "Expected a string at key: {}, got: {:?}",
                key,
                object.get(key)
            )
        })
}

pub(crate) fn idl_value_as_str_or_object_with_name_as_str_or_else(
    value: &Value,
) -> Result<&str> {
    match value.as_str() {
        Some(name) => Ok(name),
        None => {
            let object = idl_as_object_or_else(value)?;
            Ok(idl_object_get_key_as_str_or_else(object, "name")?)
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

pub(crate) fn idl_value_as_object_get_key<'a>(
    value: &'a Value,
    key: &str,
) -> Option<&'a Value> {
    value.as_object().and_then(|object| object.get(key))
}

pub(crate) fn idl_as_array_or_else(value: &Value) -> Result<&Vec<Value>> {
    value.as_array().context("Expected an array")
}

pub(crate) fn idl_as_object_or_else(
    value: &Value,
) -> Result<&Map<String, Value>> {
    value.as_object().context("Expected an object")
}

pub(crate) fn idl_as_str_or_else(value: &Value) -> Result<&str> {
    value.as_str().context("Expected an string")
}

pub(crate) fn idl_as_u64_or_else(value: &Value) -> Result<u64> {
    value.as_u64().context("Expected an unsigned number")
}

pub(crate) fn idl_as_i64_or_else(value: &Value) -> Result<i64> {
    value.as_i64().context("Expected a signed number")
}

pub(crate) fn idl_as_f64_or_else(value: &Value) -> Result<f64> {
    value.as_f64().context("Expected a floating number")
}

pub(crate) fn idl_as_bool_or_else(value: &Value) -> Result<bool> {
    value.as_bool().context("Expected a boolean")
}

pub(crate) fn idl_as_bytes_or_else(array: &[Value]) -> Result<Vec<u8>> {
    let mut bytes = vec![];
    for item in array {
        bytes.push(u8::try_from(idl_as_u64_or_else(item)?)?);
    }
    Ok(bytes)
}

pub(crate) fn idl_slice_from_bytes(
    bytes: &[u8],
    offset: usize,
    length: usize,
) -> Result<&[u8]> {
    let end = offset.checked_add(length).ok_or_else(|| {
        anyhow!(
            "Invalid slice length: offset: {}, length: {}",
            offset,
            length,
        )
    })?;
    if bytes.len() < end {
        return Err(anyhow!(
            "Invalid slice read: offset: {}, length: {}, from bytes: {}",
            offset,
            length,
            bytes.len(),
        ));
    }
    Ok(&bytes[offset..end])
}

pub(crate) fn idl_map_get_key_or_else<'a, V: std::fmt::Debug>(
    map: &'a HashMap<String, V>,
    key: &str,
) -> Result<&'a V> {
    map.get(key).ok_or_else(|| {
        anyhow!(
            "Missing key: {}, available keys: {:?}",
            key,
            map.keys().collect::<Vec<_>>()
        )
    })
}

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

pub(crate) fn idl_pubkey_from_bytes_at(
    bytes: &[u8],
    offset: usize,
) -> Result<Pubkey> {
    let size = std::mem::size_of::<Pubkey>();
    let slice = idl_slice_from_bytes(bytes, offset, size)?;
    Ok(Pubkey::new_from_array(slice.try_into()?))
}

pub(crate) fn idl_prefix_from_bytes_at(
    prefix_size: &u8,
    bytes: &[u8],
    offset: usize,
) -> Result<u32> {
    match prefix_size {
        1 => Ok(idl_u8_from_bytes_at(bytes, offset)?.into()),
        2 => Ok(idl_u16_from_bytes_at(bytes, offset)?.into()),
        4 => Ok(idl_u32_from_bytes_at(bytes, offset)?),
        _ => Err(anyhow!("Invalid prefix size: {}", prefix_size)),
    }
}

pub(crate) fn idl_prefix_write(
    prefix_size: &u8,
    value: usize,
    data: &mut Vec<u8>,
) -> Result<()> {
    match prefix_size {
        1 => {
            let value = u8::try_from(value)?;
            data.push(value);
        },
        2 => {
            let value = u16::try_from(value)?;
            data.extend_from_slice(&value.to_le_bytes());
        },
        4 => {
            let value = u32::try_from(value)?;
            data.extend_from_slice(&value.to_le_bytes());
        },
        _ => return Err(anyhow!("Invalid prefix size: {}", prefix_size)),
    }
    Ok(())
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
    let mut hasher = Sha256::new();
    hasher.update(value);
    hasher.finalize()[..8].to_vec()
}
