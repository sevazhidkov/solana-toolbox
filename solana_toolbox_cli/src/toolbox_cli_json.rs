use serde_json::{json, Map, Value};

pub fn cli_json_object_set_value_at_path(
    object: &mut Map<String, Value>,
    path: &str,
    value: Value,
) {
    // TODO - support unamed append (index array)
    if let Some((key, path_child)) = path.split_once(".") {
        if let Some(object_value) = object.get_mut(key) {
            if let Some(object_child) = object_value.as_object_mut() {
                cli_json_object_set_value_at_path(
                    object_child,
                    path_child,
                    value,
                );
                return;
            }
        }
        let mut object_child = Map::new();
        cli_json_object_set_value_at_path(&mut object_child, path_child, value);
        object.insert(key.to_string(), json!(object_child));
        return;
    }
    object.insert(path.to_string(), value);
}

pub fn cli_json_value_fit(
    superset_value: &Value,
    subset_value: &Value,
) -> bool {
    match subset_value {
        Value::Null => {
            if let Some(()) = superset_value.as_null() {
                return true;
            }
            false
        },
        Value::Bool(subset_bool) => {
            if let Some(superset_bool) = superset_value.as_bool() {
                return &superset_bool == subset_bool;
            }
            false
        },
        Value::Number(subset_number) => {
            if let Some(superset_number) = superset_value.as_number() {
                return superset_number == subset_number;
            }
            false
        },
        Value::String(subset_string) => {
            if let Some(superset_string) = superset_value.as_str() {
                return superset_string == subset_string;
            }
            false
        },
        Value::Array(subset_array) => {
            if let Some(superset_array) = superset_value.as_array() {
                if superset_array.len() < subset_array.len() {
                    return false;
                }
                for (index, subset_item) in subset_array.iter().enumerate() {
                    let superset_item = &superset_array[index];
                    if !cli_json_value_fit(superset_item, subset_item) {
                        return false;
                    }
                }
                return true;
            }
            false
        },
        Value::Object(subset_object) => {
            if let Some(superset_object) = superset_value.as_object() {
                for (key, subset_field) in subset_object {
                    if let Some(superset_field) = superset_object.get(key) {
                        if !cli_json_value_fit(superset_field, subset_field) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                return true;
            }
            false
        },
    }
}
