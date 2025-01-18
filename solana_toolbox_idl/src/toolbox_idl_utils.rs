use serde_json::Map;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
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
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<&'a Vec<Value>, ToolboxIdlError> {
    idl_ok_or_else(
        idl_object_get_key_as_array(object, key),
        "missing array at key",
        key,
        breadcrumbs,
    )
}

pub(crate) fn idl_object_get_key_as_str_or_else<'a>(
    object: &'a Map<String, Value>,
    key: &str,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<&'a str, ToolboxIdlError> {
    idl_ok_or_else(
        idl_object_get_key_as_str(object, key),
        "missing string at key",
        key,
        breadcrumbs,
    )
}

pub(crate) fn idl_object_get_key_or_else<'a>(
    object: &'a Map<String, Value>,
    key: &str,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<&'a Value, ToolboxIdlError> {
    idl_ok_or_else(object.get(key), "missing value at key", key, breadcrumbs)
}

pub(crate) fn idl_as_array_or_else<'a>(
    value: &'a Value,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<&'a Vec<Value>, ToolboxIdlError> {
    idl_ok_or_else(
        value.as_array(),
        "was expected to be of type",
        "array",
        breadcrumbs,
    )
}

pub(crate) fn idl_as_object_or_else<'a>(
    value: &'a Value,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<&'a Map<String, Value>, ToolboxIdlError> {
    idl_ok_or_else(
        value.as_object(),
        "was expected to be of type",
        "object",
        breadcrumbs,
    )
}

pub(crate) fn idl_as_str_or_else<'a>(
    value: &'a Value,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<&'a str, ToolboxIdlError> {
    idl_ok_or_else(
        value.as_str(),
        "was expected to be of type",
        "string",
        breadcrumbs,
    )
}

pub(crate) fn idl_as_u128_or_else(
    value: &Value,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<u128, ToolboxIdlError> {
    Ok(u128::from(*idl_ok_or_else(
        value.as_u64().as_ref(),
        "was expected to be of type",
        "u128",
        breadcrumbs,
    )?))
}

pub(crate) fn idl_as_i128_or_else(
    value: &Value,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<i128, ToolboxIdlError> {
    Ok(i128::from(*idl_ok_or_else(
        value.as_i64().as_ref(),
        "was expected to be of type",
        "i128",
        breadcrumbs,
    )?))
}

pub(crate) fn idl_as_bool_or_else(
    value: &Value,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<bool, ToolboxIdlError> {
    Ok(*idl_ok_or_else(
        value.as_bool().as_ref(),
        "was expected to be of type",
        "i128",
        breadcrumbs,
    )?)
}

pub(crate) fn idl_ok_or_else<'a, T: ?Sized>(
    option: Option<&'a T>,
    message_failure: &str,
    message_code: &str,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<&'a T, ToolboxIdlError> {
    option.ok_or_else(|| ToolboxIdlError::Custom {
        failure: format!("{}: {}", message_failure, message_code),
        breadcrumbs: *breadcrumbs,
    })
}

pub(crate) fn idl_err<T>(
    failure: String,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<T, ToolboxIdlError> {
    Err(ToolboxIdlError::Custom { failure, breadcrumbs: *breadcrumbs })
}

pub(crate) fn idl_slice_from_bytes<'a>(
    bytes: &'a [u8],
    offset: usize,
    length: usize,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<&'a [u8], ToolboxIdlError> {
    let end = offset.checked_add(length).ok_or_else(|| {
        ToolboxIdlError::InvalidSliceLength {
            offset,
            length,
            breadcrumbs: *breadcrumbs,
        }
    })?;
    if bytes.len() < end {
        return Err(ToolboxIdlError::InvalidSliceReadAt {
            offset,
            length,
            bytes: bytes.len(),
            breadcrumbs: *breadcrumbs,
        });
    }
    Ok(&bytes[offset..end])
}

pub(crate) fn idl_u8_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<u8, ToolboxIdlError> {
    let size = size_of::<u8>();
    let slice = idl_slice_from_bytes(bytes, offset, size, breadcrumbs)?;
    Ok(u8::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_u16_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<u16, ToolboxIdlError> {
    let size = size_of::<u16>();
    let slice = idl_slice_from_bytes(bytes, offset, size, breadcrumbs)?;
    Ok(u16::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_u32_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<u32, ToolboxIdlError> {
    let size = size_of::<u32>();
    let slice = idl_slice_from_bytes(bytes, offset, size, breadcrumbs)?;
    Ok(u32::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_u64_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<u64, ToolboxIdlError> {
    let size = size_of::<u64>();
    let slice = idl_slice_from_bytes(bytes, offset, size, breadcrumbs)?;
    Ok(u64::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_u128_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<u128, ToolboxIdlError> {
    let size = size_of::<u128>();
    let slice = idl_slice_from_bytes(bytes, offset, size, breadcrumbs)?;
    Ok(u128::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_i8_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<i8, ToolboxIdlError> {
    let size = size_of::<i8>();
    let slice = idl_slice_from_bytes(bytes, offset, size, breadcrumbs)?;
    Ok(i8::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_i16_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<i16, ToolboxIdlError> {
    let size = size_of::<i16>();
    let slice = idl_slice_from_bytes(bytes, offset, size, breadcrumbs)?;
    Ok(i16::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_i32_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<i32, ToolboxIdlError> {
    let size = size_of::<i32>();
    let slice = idl_slice_from_bytes(bytes, offset, size, breadcrumbs)?;
    Ok(i32::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_i64_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<i64, ToolboxIdlError> {
    let size = size_of::<i64>();
    let slice = idl_slice_from_bytes(bytes, offset, size, breadcrumbs)?;
    Ok(i64::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_i128_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<i128, ToolboxIdlError> {
    let size = size_of::<i128>();
    let slice = idl_slice_from_bytes(bytes, offset, size, breadcrumbs)?;
    Ok(i128::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_pubkey_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<Pubkey, ToolboxIdlError> {
    let size = size_of::<Pubkey>();
    let slice = idl_slice_from_bytes(bytes, offset, size, breadcrumbs)?;
    Ok(Pubkey::new_from_array(slice.try_into().unwrap()))
}
