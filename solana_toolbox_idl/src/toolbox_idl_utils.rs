use std::collections::HashMap;

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

pub(crate) fn idl_object_get_key_as_object_or_else<'a>(
    object: &'a Map<String, Value>,
    key: &str,
    context: &ToolboxIdlContext,
) -> Result<&'a Map<String, Value>, ToolboxIdlError> {
    idl_ok_or_else(
        idl_object_get_key_as_object(object, key),
        &format!("expected an object at key: {}", key),
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

type ScopedObject<'a> = (&'a Map<String, Value>, ToolboxIdlBreadcrumbs);

pub(crate) fn idl_object_get_key_as_scoped_object_array_or_else<'a>(
    object: &'a Map<String, Value>,
    key: &str,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<Vec<ScopedObject<'a>>, ToolboxIdlError> {
    let items_array =
        idl_object_get_key_as_array_or_else(object, key, &breadcrumbs.idl())?;
    let breadcrumbs = &breadcrumbs.with_idl(key);
    let mut items_object_array = vec![];
    for item_index in 0..items_array.len() {
        let item_value = items_array.get(item_index).unwrap();
        let item_tag = format!("[{}]", item_index);
        let item_object =
            idl_as_object_or_else(item_value, &breadcrumbs.as_idl(&item_tag))?;
        items_object_array.push((item_object, breadcrumbs.with_idl(&item_tag)));
    }
    Ok(items_object_array)
}

type ScopedNamedObject<'a> =
    (&'a str, &'a Map<String, Value>, ToolboxIdlBreadcrumbs);

// TODO - could specialize this a bit, args cannot be objects and enum variants could be array of string ?
pub(crate) fn idl_object_get_key_as_scoped_named_object_array_or_else<'a>(
    object: &'a Map<String, Value>,
    key: &str,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<Vec<ScopedNamedObject<'a>>, ToolboxIdlError> {
    if let Some(items_object) = idl_object_get_key_as_object(object, key) {
        let breadcrumbs = &breadcrumbs.with_idl(key);
        let mut items_named_object_array = vec![];
        for (item_name, item_value) in items_object {
            let item_object = idl_as_object_or_else(
                item_value,
                &breadcrumbs.as_idl(item_name),
            )?;
            items_named_object_array.push((
                item_name.as_str(),
                item_object,
                breadcrumbs.with_idl(item_name),
            ));
        }
        return Ok(items_named_object_array);
    }
    let items_array =
        idl_object_get_key_as_array_or_else(object, key, &breadcrumbs.idl())?;
    let breadcrumbs = &breadcrumbs.with_idl(key);
    let mut items_named_object_array = vec![];
    for item_index in 0..items_array.len() {
        let item_value = items_array.get(item_index).unwrap();
        let item_tag = format!("[{}]", item_index);
        let item_object =
            idl_as_object_or_else(item_value, &breadcrumbs.as_idl(&item_tag))?;
        let item_name = idl_object_get_key_as_str_or_else(
            item_object,
            "name",
            &breadcrumbs.as_idl(&item_tag),
        )?;
        items_named_object_array.push((
            item_name,
            item_object,
            breadcrumbs.with_idl(item_name),
        ));
    }
    Ok(items_named_object_array)
}

type ScopedNamedContentValue<'a> = (&'a str, &'a Value, ToolboxIdlBreadcrumbs);

pub(crate) fn idl_object_get_key_as_scoped_named_content_array_or_else<'a>(
    object: &'a Map<String, Value>,
    key: &str,
    content_key: &str,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<Vec<ScopedNamedContentValue<'a>>, ToolboxIdlError> {
    let mut items_named_inner_object_array = vec![];
    for (idl_item_name, idl_item_object, breadcrumbs) in
        idl_object_get_key_as_scoped_named_object_array_or_else(
            object,
            key,
            breadcrumbs,
        )?
    {
        let idl_item_content = idl_object_get_key_or_else(
            idl_item_object,
            content_key,
            &breadcrumbs.idl(),
        )?;
        items_named_inner_object_array.push((
            idl_item_name,
            idl_item_content,
            breadcrumbs,
        ))
    }
    Ok(items_named_inner_object_array)
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
    value: &Value,
    context: &ToolboxIdlContext,
) -> Result<Vec<u8>, ToolboxIdlError> {
    let mut bytes = vec![];
    let array = idl_as_array_or_else(value, context)?;
    for index in 0..array.len() {
        let item = array.get(index).unwrap();
        let integer = idl_as_u128_or_else(item, context)?;
        let byte = u8::try_from(integer).map_err(|err| {
            ToolboxIdlError::InvalidInteger {
                conversion: err,
                context: context.clone(),
            }
        })?;
        bytes.push(byte);
    }
    Ok(bytes)
}

pub(crate) fn idl_map_get_key_or_else<'a, V>(
    map: &'a HashMap<String, V>,
    key: &str,
    context: &ToolboxIdlContext,
) -> Result<&'a V, ToolboxIdlError> {
    idl_ok_or_else(map.get(key), &format!("missing key: {}", key), context)
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
    let size = size_of::<u8>();
    let slice = idl_slice_from_bytes(bytes, offset, size, context)?;
    Ok(u8::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_u16_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    context: &ToolboxIdlContext,
) -> Result<u16, ToolboxIdlError> {
    let size = size_of::<u16>();
    let slice = idl_slice_from_bytes(bytes, offset, size, context)?;
    Ok(u16::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_u32_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    context: &ToolboxIdlContext,
) -> Result<u32, ToolboxIdlError> {
    let size = size_of::<u32>();
    let slice = idl_slice_from_bytes(bytes, offset, size, context)?;
    Ok(u32::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_u64_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    context: &ToolboxIdlContext,
) -> Result<u64, ToolboxIdlError> {
    let size = size_of::<u64>();
    let slice = idl_slice_from_bytes(bytes, offset, size, context)?;
    Ok(u64::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_u128_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    context: &ToolboxIdlContext,
) -> Result<u128, ToolboxIdlError> {
    let size = size_of::<u128>();
    let slice = idl_slice_from_bytes(bytes, offset, size, context)?;
    Ok(u128::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_i8_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    context: &ToolboxIdlContext,
) -> Result<i8, ToolboxIdlError> {
    let size = size_of::<i8>();
    let slice = idl_slice_from_bytes(bytes, offset, size, context)?;
    Ok(i8::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_i16_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    context: &ToolboxIdlContext,
) -> Result<i16, ToolboxIdlError> {
    let size = size_of::<i16>();
    let slice = idl_slice_from_bytes(bytes, offset, size, context)?;
    Ok(i16::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_i32_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    context: &ToolboxIdlContext,
) -> Result<i32, ToolboxIdlError> {
    let size = size_of::<i32>();
    let slice = idl_slice_from_bytes(bytes, offset, size, context)?;
    Ok(i32::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_i64_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    context: &ToolboxIdlContext,
) -> Result<i64, ToolboxIdlError> {
    let size = size_of::<i64>();
    let slice = idl_slice_from_bytes(bytes, offset, size, context)?;
    Ok(i64::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_i128_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    context: &ToolboxIdlContext,
) -> Result<i128, ToolboxIdlError> {
    let size = size_of::<i128>();
    let slice = idl_slice_from_bytes(bytes, offset, size, context)?;
    Ok(i128::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_f32_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    context: &ToolboxIdlContext,
) -> Result<f32, ToolboxIdlError> {
    let size = size_of::<f32>();
    let slice = idl_slice_from_bytes(bytes, offset, size, context)?;
    Ok(f32::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_f64_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    context: &ToolboxIdlContext,
) -> Result<f64, ToolboxIdlError> {
    let size = size_of::<f32>();
    let slice = idl_slice_from_bytes(bytes, offset, size, context)?;
    Ok(f64::from_le_bytes(slice.try_into().unwrap()))
}

pub(crate) fn idl_pubkey_from_bytes_at(
    bytes: &[u8],
    offset: usize,
    context: &ToolboxIdlContext,
) -> Result<Pubkey, ToolboxIdlError> {
    let size = size_of::<Pubkey>();
    let slice = idl_slice_from_bytes(bytes, offset, size, context)?;
    Ok(Pubkey::new_from_array(slice.try_into().unwrap()))
}

// TODO - there should probably be a visitor type pattern for the IDL types since its so complicated and duplicated ?
pub(crate) fn idl_describe_type(
    idl_type: &Value,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<String, ToolboxIdlError> {
    if let Some(idl_type_object) = idl_type.as_object() {
        if let Some(idl_type_defined) = idl_type_object.get("defined") {
            return Ok(idl_value_as_str_or_object_with_name_as_str_or_else(
                idl_type_defined,
                &breadcrumbs.as_idl("defined"),
            )?
            .to_string());
        }
        if let Some(idl_type_option) = idl_type_object.get("option") {
            return Ok(format!(
                "Option<{}>",
                idl_describe_type(
                    idl_type_option,
                    &breadcrumbs.with_idl("Option"),
                )?
            ));
        }
        // TODO - support for shorthand on kind using known keys
        if let Some(idl_type_kind) =
            idl_object_get_key_as_str(idl_type_object, "kind")
        {
            if idl_type_kind == "struct" {
                return Ok("Struct(?)".to_string());
            }
            if idl_type_kind == "enum" {
                return Ok("Enum(?)".to_string());
            }
        }
        if let Some(idl_type_array) =
            idl_object_get_key_as_array(idl_type_object, "array")
        {
            if idl_type_array.len() != 2 {
                return Ok("unparsable array".to_string());
            }
            return Ok(format!(
                "[{}; {}]",
                idl_describe_type(
                    &idl_type_array[0],
                    &breadcrumbs.with_idl("Array")
                )?,
                idl_type_array[1]
            ));
        }
        if let Some(idl_type_vec) = idl_type_object.get("vec") {
            return Ok(format!(
                "Vec<{}>",
                idl_describe_type(idl_type_vec, &breadcrumbs.with_idl("Vec"))?
            ));
        }
    }
    // TODO - support for array/vec shorthand and leaf shorthand
    if let Some(idl_type_leaf) = idl_type.as_str() {
        return Ok(idl_type_leaf.to_string());
    }
    Ok("unparsable type".to_string())
}
