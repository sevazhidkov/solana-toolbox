use serde_json::Map;
use serde_json::Value;

use crate::ToolboxAnchorError;

pub(crate) fn idl_object_get_key_as_array<'a>(
    object: &'a Map<String, Value>,
    key: &str,
) -> Option<&'a Vec<Value>> {
    object.get(key).map(|value| value.as_array()).flatten()
}

pub(crate) fn idl_object_get_key_as_object<'a>(
    object: &'a Map<String, Value>,
    key: &str,
) -> Option<&'a Map<String, Value>> {
    object.get(key).map(|value| value.as_object()).flatten()
}

pub(crate) fn idl_object_get_key_as_str<'a>(
    object: &'a Map<String, Value>,
    key: &str,
) -> Option<&'a str> {
    object.get(key).map(|value| value.as_str()).flatten()
}

pub(crate) fn idl_object_get_key_as_bool(
    object: &Map<String, Value>,
    key: &str,
) -> Option<bool> {
    object.get(key).map(|value| value.as_bool()).flatten()
}

pub(crate) fn idl_object_get_key_as_array_or_else<'a>(
    object: &'a Map<String, Value>,
    key: &str,
    context: &str,
) -> Result<&'a Vec<Value>, ToolboxAnchorError> {
    idl_ok_or_else(
        idl_object_get_key_as_array(object, key),
        context,
        "missing array at key",
        key,
        object,
    )
}

pub(crate) fn idl_object_get_key_as_object_or_else<'a>(
    object: &'a Map<String, Value>,
    key: &str,
    context: &str,
) -> Result<&'a Map<String, Value>, ToolboxAnchorError> {
    idl_ok_or_else(
        idl_object_get_key_as_object(object, key),
        context,
        "missing object at key",
        key,
        object,
    )
}

pub(crate) fn idl_object_get_key_as_str_or_else<'a>(
    object: &'a Map<String, Value>,
    key: &str,
    context: &str,
) -> Result<&'a str, ToolboxAnchorError> {
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
) -> Result<&'a Value, ToolboxAnchorError> {
    idl_ok_or_else(
        object.get(key),
        context,
        "missing value at key",
        key,
        object,
    )
}

pub(crate) fn idl_as_object_or_else<'a>(
    value: &'a Value,
    context: &str,
) -> Result<&'a Map<String, Value>, ToolboxAnchorError> {
    idl_ok_or_else(
        value.as_object(),
        context,
        "was expected to be of type",
        "object",
        value,
    )
}

pub(crate) fn idl_as_u64_or_else<'a>(
    value: &'a Value,
    context: &str,
) -> Result<u64, ToolboxAnchorError> {
    idl_ok_or_else(
        value.as_u64().as_ref(),
        context,
        "was expected to be of type",
        "object",
        value,
    )
    .cloned()
}

pub(crate) fn idl_ok_or_else<'a, T: ?Sized, P: std::fmt::Debug>(
    option: Option<&'a T>,
    context_kind: &str,
    context_msg: &str,
    context_code: &str,
    param: &P,
) -> Result<&'a T, ToolboxAnchorError> {
    option.ok_or_else(|| {
        ToolboxAnchorError::Custom(format!(
            "IDL: {}: {}: {}: {:?}",
            context_kind, context_msg, context_code, param
        ))
    })
}

pub(crate) fn idl_err<T, P: std::fmt::Debug>(
    context_msg: &str,
    context_code: &str,
    param: &P,
) -> Result<T, ToolboxAnchorError> {
    Err(ToolboxAnchorError::Custom(format!(
        "IDL: {}: {}: {:?}",
        context_msg, context_code, param
    )))
}
