use std::collections::HashMap;
use std::num::TryFromIntError;

use serde_json::Map;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_context::ToolboxIdlContext;
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

pub(crate) fn idl_object_get_key_as_array_or_else<'a>(
    object: &'a Map<String, Value>,
    key: &str,
    context: &ToolboxIdlContext,
) -> Result<&'a Vec<Value>, ToolboxIdlError> {
    idl_ok_or_else(
        idl_object_get_key_as_array(object, key),
        &format!("expected an array at key: {}", key),
        context,
    )
}

pub(crate) fn idl_object_get_key_as_str_or_else<'a>(
    object: &'a Map<String, Value>,
    key: &str,
    context: &ToolboxIdlContext,
) -> Result<&'a str, ToolboxIdlError> {
    idl_ok_or_else(
        idl_object_get_key_as_str(object, key),
        &format!("expected a string at key: {}", key),
        context,
    )
}

pub(crate) fn idl_object_get_key_as_u64_or_else(
    object: &Map<String, Value>,
    key: &str,
    context: &ToolboxIdlContext,
) -> Result<u64, ToolboxIdlError> {
    Ok(*idl_ok_or_else(
        idl_object_get_key_as_u64(object, key).as_ref(),
        &format!("expected a string at key: {}", key),
        context,
    )?)
}

pub(crate) fn idl_object_get_key_or_else<'a>(
    object: &'a Map<String, Value>,
    key: &str,
    context: &ToolboxIdlContext,
) -> Result<&'a Value, ToolboxIdlError> {
    idl_ok_or_else(
        object.get(key),
        &format!("missing value at key: {}", key),
        context,
    )
}

// TODO - used in program_instruct (could be inlined there, and program_idl, could be modified/inlined)
type ScopedNamedObject<'a> =
    (&'a str, &'a Map<String, Value>, ToolboxIdlBreadcrumbs);
pub(crate) fn idl_array_get_scoped_named_object_array_or_else<'a>(
    idl_array: &'a [Value],
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<Vec<ScopedNamedObject<'a>>, ToolboxIdlError> {
    let mut scoped_named_object_array = vec![];
    for (index, idl_item) in idl_array.iter().enumerate() {
        let context = &breadcrumbs.as_idl(&format!("[{}]", index));
        let idl_item = idl_as_object_or_else(idl_item, context)?;
        let idl_item_name =
            idl_object_get_key_as_str_or_else(idl_item, "name", context)?;
        scoped_named_object_array.push((
            idl_item_name,
            idl_item,
            breadcrumbs.with_idl(idl_item_name),
        ))
    }
    Ok(scoped_named_object_array)
}

// TODO - only used in program_idl, where it can be modified/inlined
type ScopedKeyValue<'a> = (&'a str, &'a Value, ToolboxIdlBreadcrumbs);
pub(crate) fn idl_object_get_scoped_key_value_array<'a>(
    idl_object: &'a Map<String, Value>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<Vec<ScopedKeyValue<'a>>, ToolboxIdlError> {
    let mut scoped_key_value_array = vec![];
    for (idl_key, idl_value) in idl_object {
        scoped_key_value_array.push((
            idl_key.as_str(),
            idl_value,
            breadcrumbs.with_idl(idl_key),
        ))
    }
    Ok(scoped_key_value_array)
}

pub(crate) fn idl_value_as_str_or_object_with_name_as_str_or_else<'a>(
    value: &'a Value,
    context: &ToolboxIdlContext,
) -> Result<&'a str, ToolboxIdlError> {
    match value.as_str() {
        Some(name) => Ok(name),
        None => {
            let object = idl_as_object_or_else(value, context)?;
            Ok(idl_object_get_key_as_str_or_else(object, "name", context)?)
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

pub(crate) fn idl_as_array_or_else<'a>(
    value: &'a Value,
    context: &ToolboxIdlContext,
) -> Result<&'a Vec<Value>, ToolboxIdlError> {
    idl_ok_or_else(value.as_array(), "expected an array", context)
}

pub(crate) fn idl_as_object_or_else<'a>(
    value: &'a Value,
    context: &ToolboxIdlContext,
) -> Result<&'a Map<String, Value>, ToolboxIdlError> {
    idl_ok_or_else(value.as_object(), "expected an object", context)
}

pub(crate) fn idl_as_str_or_else<'a>(
    value: &'a Value,
    context: &ToolboxIdlContext,
) -> Result<&'a str, ToolboxIdlError> {
    idl_ok_or_else(value.as_str(), "expected a string", context)
}

pub(crate) fn idl_as_u128_or_else(
    value: &Value,
    context: &ToolboxIdlContext,
) -> Result<u128, ToolboxIdlError> {
    Ok(u128::from(*idl_ok_or_else(
        value.as_u64().as_ref(),
        "expected an unsigned number",
        context,
    )?))
}

pub(crate) fn idl_as_i128_or_else(
    value: &Value,
    context: &ToolboxIdlContext,
) -> Result<i128, ToolboxIdlError> {
    Ok(i128::from(*idl_ok_or_else(
        value.as_i64().as_ref(),
        "expected a signed number",
        context,
    )?))
}

pub(crate) fn idl_as_f64_or_else(
    value: &Value,
    context: &ToolboxIdlContext,
) -> Result<f64, ToolboxIdlError> {
    Ok(*idl_ok_or_else(
        value.as_f64().as_ref(),
        "expected a floating number",
        context,
    )?)
}

pub(crate) fn idl_as_bool_or_else(
    value: &Value,
    context: &ToolboxIdlContext,
) -> Result<bool, ToolboxIdlError> {
    Ok(*idl_ok_or_else(
        value.as_bool().as_ref(),
        "expected a boolean",
        context,
    )?)
}

pub(crate) fn idl_as_bytes_or_else(
    array: &[Value],
    context: &ToolboxIdlContext,
) -> Result<Vec<u8>, ToolboxIdlError> {
    let mut bytes = vec![];
    for item in array {
        let integer = idl_as_u128_or_else(item, context)?;
        let byte = idl_map_err_invalid_integer(u8::try_from(integer), context)?;
        bytes.push(byte);
    }
    Ok(bytes)
}

pub(crate) fn idl_ok_or_else<'a, T: ?Sized>(
    option: Option<&'a T>,
    failure: &str,
    context: &ToolboxIdlContext,
) -> Result<&'a T, ToolboxIdlError> {
    option.ok_or_else(|| {
        ToolboxIdlError::Custom {
            failure: failure.to_string(),
            context: context.clone(),
        }
    })
}

pub(crate) fn idl_err<T>(
    failure: &str,
    context: &ToolboxIdlContext,
) -> Result<T, ToolboxIdlError> {
    Err(ToolboxIdlError::Custom {
        failure: failure.to_string(),
        context: context.clone(),
    })
}

pub(crate) fn idl_slice_from_bytes<'a>(
    bytes: &'a [u8],
    offset: usize,
    length: usize,
    context: &ToolboxIdlContext,
) -> Result<&'a [u8], ToolboxIdlError> {
    let end = offset.checked_add(length).ok_or_else(|| {
        ToolboxIdlError::InvalidSliceLength {
            offset,
            length,
            context: context.clone(),
        }
    })?;
    if bytes.len() < end {
        return Err(ToolboxIdlError::InvalidSliceReadAt {
            offset,
            length,
            bytes: bytes.len(),
            context: context.clone(),
        });
    }
    Ok(&bytes[offset..end])
}

pub(crate) fn idl_u8_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    context: &ToolboxIdlContext,
) -> Result<u8, ToolboxIdlError> {
    let size = std::mem::size_of::<u8>();
    let slice = idl_slice_from_bytes(bytes, offset, size, context)?;
    Ok(u8::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_u16_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    context: &ToolboxIdlContext,
) -> Result<u16, ToolboxIdlError> {
    let size = std::mem::size_of::<u16>();
    let slice = idl_slice_from_bytes(bytes, offset, size, context)?;
    Ok(u16::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_u32_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    context: &ToolboxIdlContext,
) -> Result<u32, ToolboxIdlError> {
    let size = std::mem::size_of::<u32>();
    let slice = idl_slice_from_bytes(bytes, offset, size, context)?;
    Ok(u32::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_u64_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    context: &ToolboxIdlContext,
) -> Result<u64, ToolboxIdlError> {
    let size = std::mem::size_of::<u64>();
    let slice = idl_slice_from_bytes(bytes, offset, size, context)?;
    Ok(u64::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_u128_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    context: &ToolboxIdlContext,
) -> Result<u128, ToolboxIdlError> {
    let size = std::mem::size_of::<u128>();
    let slice = idl_slice_from_bytes(bytes, offset, size, context)?;
    Ok(u128::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_i8_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    context: &ToolboxIdlContext,
) -> Result<i8, ToolboxIdlError> {
    let size = std::mem::size_of::<i8>();
    let slice = idl_slice_from_bytes(bytes, offset, size, context)?;
    Ok(i8::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_i16_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    context: &ToolboxIdlContext,
) -> Result<i16, ToolboxIdlError> {
    let size = std::mem::size_of::<i16>();
    let slice = idl_slice_from_bytes(bytes, offset, size, context)?;
    Ok(i16::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_i32_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    context: &ToolboxIdlContext,
) -> Result<i32, ToolboxIdlError> {
    let size = std::mem::size_of::<i32>();
    let slice = idl_slice_from_bytes(bytes, offset, size, context)?;
    Ok(i32::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_i64_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    context: &ToolboxIdlContext,
) -> Result<i64, ToolboxIdlError> {
    let size = std::mem::size_of::<i64>();
    let slice = idl_slice_from_bytes(bytes, offset, size, context)?;
    Ok(i64::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_i128_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    context: &ToolboxIdlContext,
) -> Result<i128, ToolboxIdlError> {
    let size = std::mem::size_of::<i128>();
    let slice = idl_slice_from_bytes(bytes, offset, size, context)?;
    Ok(i128::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_f32_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    context: &ToolboxIdlContext,
) -> Result<f32, ToolboxIdlError> {
    let size = std::mem::size_of::<f32>();
    let slice = idl_slice_from_bytes(bytes, offset, size, context)?;
    Ok(f32::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_f64_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    context: &ToolboxIdlContext,
) -> Result<f64, ToolboxIdlError> {
    let size = std::mem::size_of::<f32>();
    let slice = idl_slice_from_bytes(bytes, offset, size, context)?;
    Ok(f64::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_pubkey_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    context: &ToolboxIdlContext,
) -> Result<Pubkey, ToolboxIdlError> {
    let size = std::mem::size_of::<Pubkey>();
    let slice = idl_slice_from_bytes(bytes, offset, size, context)?;
    Ok(Pubkey::new_from_array(slice.try_into().unwrap()))
}

// TODO - could be clean'ed
pub(crate) fn idl_map_get_key_or_else<'a, V: std::fmt::Debug>(
    map: &'a HashMap<String, V>,
    key: &str,
    context: &ToolboxIdlContext,
) -> Result<&'a V, ToolboxIdlError> {
    idl_ok_or_else(map.get(key), &format!("missing key: {}", key), context)
}

pub(crate) fn idl_map_err_invalid_integer<V>(
    result: Result<V, TryFromIntError>,
    context: &ToolboxIdlContext,
) -> Result<V, ToolboxIdlError> {
    result.map_err(|err| {
        ToolboxIdlError::InvalidInteger {
            conversion: err,
            context: context.clone(),
        }
    })
}

pub(crate) fn idl_str_to_usize_or_else(
    value: &str,
    context: &ToolboxIdlContext,
) -> Result<usize, ToolboxIdlError> {
    value.parse().map_err(|err| {
        ToolboxIdlError::InvalidNumber {
            parsing: err,
            context: context.clone(),
        }
    })
}

pub(crate) fn idl_iter_get_scoped_values<'a, T>(
    iter: impl IntoIterator<Item = &'a T>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<Vec<(usize, &'a T, ToolboxIdlBreadcrumbs)>, ToolboxIdlError> {
    let mut scoped_values = vec![];
    for (item_index, item) in iter.into_iter().enumerate() {
        scoped_values.push((
            item_index,
            item,
            breadcrumbs.with_idl(&format!("[{}]", item_index)),
        ));
    }
    Ok(scoped_values)
}
