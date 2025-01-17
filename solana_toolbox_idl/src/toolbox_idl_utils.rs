use serde_json::Map;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_error::ToolboxIdlError;

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

pub(crate) fn idl_object_get_key_as_bool(
    object: &Map<String, Value>,
    key: &str,
) -> Option<bool> {
    object.get(key).and_then(|value| value.as_bool())
}

pub(crate) fn idl_object_get_key_as_array_or_else<'a>(
    object: &'a Map<String, Value>,
    key: &str,
    context: &str,
) -> Result<&'a Vec<Value>, ToolboxIdlError> {
    idl_ok_or_else(
        idl_object_get_key_as_array(object, key),
        context,
        "missing array at key",
        key,
        object,
    )
}

pub(crate) fn idl_object_get_key_as_str_or_else<'a>(
    object: &'a Map<String, Value>,
    key: &str,
    context: &str,
) -> Result<&'a str, ToolboxIdlError> {
    idl_ok_or_else(
        idl_object_get_key_as_str(object, key),
        context,
        "missing string at key",
        key,
        object,
    )
}

pub(crate) fn idl_object_get_key_or_else<'a>(
    object: &'a Map<String, Value>,
    key: &str,
    context: &str,
) -> Result<&'a Value, ToolboxIdlError> {
    idl_ok_or_else(
        object.get(key),
        context,
        "missing value at key",
        key,
        object,
    )
}

pub(crate) fn idl_as_array_or_else<'a>(
    value: &'a Value,
    context: &str,
) -> Result<&'a Vec<Value>, ToolboxIdlError> {
    idl_ok_or_else(
        value.as_array(),
        context,
        "was expected to be of type",
        "array",
        value,
    )
}

pub(crate) fn idl_as_object_or_else<'a>(
    value: &'a Value,
    context: &str,
) -> Result<&'a Map<String, Value>, ToolboxIdlError> {
    idl_ok_or_else(
        value.as_object(),
        context,
        "was expected to be of type",
        "object",
        value,
    )
}

pub(crate) fn idl_as_str_or_else<'a>(
    value: &'a Value,
    context: &str,
) -> Result<&'a str, ToolboxIdlError> {
    idl_ok_or_else(
        value.as_str(),
        context,
        "was expected to be of type",
        "string",
        value,
    )
}

pub(crate) fn idl_as_u128_or_else(
    value: &Value,
    context: &str,
) -> Result<u128, ToolboxIdlError> {
    Ok(u128::from(*idl_ok_or_else(
        value.as_u64().as_ref(),
        context,
        "was expected to be of type",
        "u128",
        value,
    )?))
}

pub(crate) fn idl_as_i128_or_else(
    value: &Value,
    context: &str,
) -> Result<i128, ToolboxIdlError> {
    Ok(i128::from(*idl_ok_or_else(
        value.as_i64().as_ref(),
        context,
        "was expected to be of type",
        "i128",
        value,
    )?))
}

pub(crate) fn idl_as_bool_or_else(
    value: &Value,
    context: &str,
) -> Result<bool, ToolboxIdlError> {
    Ok(*idl_ok_or_else(
        value.as_bool().as_ref(),
        context,
        "was expected to be of type",
        "i128",
        value,
    )?)
}

pub(crate) fn idl_ok_or_else<'a, T: ?Sized, P: std::fmt::Debug>(
    option: Option<&'a T>,
    message_context: &str,
    message_error: &str,
    message_key: &str,
    param: &P,
) -> Result<&'a T, ToolboxIdlError> {
    option.ok_or_else(|| {
        ToolboxIdlError::Custom(format!(
            "IDL: {}: {}: {}: {:?}",
            message_context, message_error, message_key, param
        ))
    })
}

pub(crate) fn idl_err<T>(context: &str) -> Result<T, ToolboxIdlError> {
    Err(ToolboxIdlError::Custom(format!("IDL: {}", context)))
}

pub(crate) fn idl_slice_from_bytes<'a>(
    bytes: &'a [u8],
    offset: usize,
    length: usize,
) -> Result<&'a [u8], ToolboxIdlError> {
    let end =
        offset.checked_add(length).ok_or_else(ToolboxIdlError::Overflow)?;
    if bytes.len() < end {
        return idl_err(&format!(
            "Unable to read bytes {} at offset {} (on byte slice of lenght {})",
            length,
            offset,
            bytes.len()
        ));
    }
    Ok(&bytes[offset..end])
}

pub(crate) fn idl_u8_from_bytes_at(
    bytes: &[u8],
    offset: usize,
) -> Result<u8, ToolboxIdlError> {
    let size = size_of::<u8>();
    let slice = idl_slice_from_bytes(bytes, offset, size)?;
    Ok(u8::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_u16_from_bytes_at(
    bytes: &[u8],
    offset: usize,
) -> Result<u16, ToolboxIdlError> {
    let size = size_of::<u16>();
    let slice = idl_slice_from_bytes(bytes, offset, size)?;
    Ok(u16::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_u32_from_bytes_at(
    bytes: &[u8],
    offset: usize,
) -> Result<u32, ToolboxIdlError> {
    let size = size_of::<u32>();
    let slice = idl_slice_from_bytes(bytes, offset, size)?;
    Ok(u32::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_u64_from_bytes_at(
    bytes: &[u8],
    offset: usize,
) -> Result<u64, ToolboxIdlError> {
    let size = size_of::<u64>();
    let slice = idl_slice_from_bytes(bytes, offset, size)?;
    Ok(u64::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_u128_from_bytes_at(
    bytes: &[u8],
    offset: usize,
) -> Result<u128, ToolboxIdlError> {
    let size = size_of::<u128>();
    let slice = idl_slice_from_bytes(bytes, offset, size)?;
    Ok(u128::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_i8_from_bytes_at(
    bytes: &[u8],
    offset: usize,
) -> Result<i8, ToolboxIdlError> {
    let size = size_of::<i8>();
    let slice = idl_slice_from_bytes(bytes, offset, size)?;
    Ok(i8::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_i16_from_bytes_at(
    bytes: &[u8],
    offset: usize,
) -> Result<i16, ToolboxIdlError> {
    let size = size_of::<i16>();
    let slice = idl_slice_from_bytes(bytes, offset, size)?;
    Ok(i16::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_i32_from_bytes_at(
    bytes: &[u8],
    offset: usize,
) -> Result<i32, ToolboxIdlError> {
    let size = size_of::<i32>();
    let slice = idl_slice_from_bytes(bytes, offset, size)?;
    Ok(i32::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_i64_from_bytes_at(
    bytes: &[u8],
    offset: usize,
) -> Result<i64, ToolboxIdlError> {
    let size = size_of::<i64>();
    let slice = idl_slice_from_bytes(bytes, offset, size)?;
    Ok(i64::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_i128_from_bytes_at(
    bytes: &[u8],
    offset: usize,
) -> Result<i128, ToolboxIdlError> {
    let size = size_of::<i128>();
    let slice = idl_slice_from_bytes(bytes, offset, size)?;
    Ok(i128::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_pubkey_from_bytes_at(
    bytes: &[u8],
    offset: usize,
) -> Result<Pubkey, ToolboxIdlError> {
    let size = size_of::<Pubkey>();
    let slice = idl_slice_from_bytes(bytes, offset, size)?;
    Ok(Pubkey::new_from_array(slice.try_into().unwrap()))
}
