use serde_json::Map;
use serde_json::Value;

pub(crate) fn json_object_get_key_as_array<'a>(
    object: &'a Map<String, Value>,
    key: &str,
) -> Option<&'a Vec<Value>> {
    object.get(key).map(|value| value.as_array()).flatten()
}

pub(crate) fn json_object_get_key_as_object<'a>(
    object: &'a Map<String, Value>,
    key: &str,
) -> Option<&'a Map<String, Value>> {
    object.get(key).map(|value| value.as_object()).flatten()
}

pub(crate) fn json_object_get_key_as_str<'a>(
    object: &'a Map<String, Value>,
    key: &str,
) -> Option<&'a str> {
    object.get(key).map(|value| value.as_str()).flatten()
}

pub(crate) fn json_object_get_key_as_bool(
    object: &Map<String, Value>,
    key: &str,
) -> Option<bool> {
    object.get(key).map(|value| value.as_bool()).flatten()
}
