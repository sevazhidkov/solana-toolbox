use anyhow::anyhow;
use anyhow::Result;
use serde_json::json;
use serde_json::Value;

use crate::toolbox_idl_path::ToolboxIdlPath;
use crate::toolbox_idl_path::ToolboxIdlPathPart;

impl ToolboxIdlPath {
    pub fn try_get_json_value<'a>(
        &self,
        value: &'a Value,
    ) -> Result<&'a Value> {
        let Some((current, next)) = self.split_first() else {
            return Ok(value);
        };
        match value {
            Value::Null => Err(anyhow!(
                "Null value does not contain path: {}",
                self.value()
            )),
            Value::Bool(_) => Err(anyhow!(
                "Bool value does not contain path: {}",
                self.value()
            )),
            Value::Number(_) => Err(anyhow!(
                "Number value does not contain path: {}",
                self.value()
            )),
            Value::String(_) => Err(anyhow!(
                "String value does not contain path: {}",
                self.value()
            )),
            Value::Array(values) => {
                let length = values.len();
                let index = match current {
                    ToolboxIdlPathPart::Empty => 0,
                    ToolboxIdlPathPart::Key(key) => {
                        return Err(anyhow!("Invalid Array Index: {}", key));
                    },
                    ToolboxIdlPathPart::Code(code) => usize::try_from(code)?,
                };
                if index >= length {
                    return Err(anyhow!(
                        "Invalid array index: {} (length: {})",
                        index,
                        length
                    ));
                }
                next.try_get_json_value(&values[index])
            },
            Value::Object(map) => {
                let key = current.value();
                for (object_key, object_value) in map {
                    if object_key == &key {
                        return next.try_get_json_value(object_value);
                    }
                }
                Err(anyhow!("Could not find object key: {}", key))
            },
        }
    }

    pub fn try_set_json_value(
        &self,
        node: Option<Value>,
        leaf: Value,
    ) -> Result<Value> {
        let Some((current, next)) = self.split_first() else {
            return Ok(leaf);
        };
        match node {
            Some(Value::Null) => Err(anyhow!(
                "Null value does not contain path: {}",
                self.value()
            )),
            Some(Value::Bool(_)) => Err(anyhow!(
                "Bool value does not contain path: {}",
                self.value()
            )),
            Some(Value::Number(_)) => Err(anyhow!(
                "Number value does not contain path: {}",
                self.value()
            )),
            Some(Value::String(_)) => Err(anyhow!(
                "String value does not contain path: {}",
                self.value()
            )),
            Some(Value::Array(mut values)) => {
                let length = values.len();
                let index = match current {
                    ToolboxIdlPathPart::Empty => length,
                    ToolboxIdlPathPart::Key(key) => {
                        return Err(anyhow!("Invalid Array Index: {}", key));
                    },
                    ToolboxIdlPathPart::Code(code) => usize::try_from(code)?,
                };
                if index > length {
                    return Err(anyhow!(
                        "Invalid array index: {} (length: {})",
                        index,
                        length
                    ));
                }
                let node = if index == length {
                    None
                } else {
                    Some(values.remove(index))
                };
                values.insert(index, next.try_set_json_value(node, leaf)?);
                Ok(json!(values))
            },
            Some(Value::Object(map)) => {
                let key = current.value();
                let mut map = map.clone();
                let node = map.remove(&key);
                map.insert(
                    key.to_string(),
                    next.try_set_json_value(node, leaf)?,
                );
                Ok(json!(map))
            },
            None => match current {
                ToolboxIdlPathPart::Empty => {
                    self.try_set_json_value(Some(json!([])), leaf)
                },
                ToolboxIdlPathPart::Key(_) => {
                    self.try_set_json_value(Some(json!({})), leaf)
                },
                ToolboxIdlPathPart::Code(_) => {
                    self.try_set_json_value(Some(json!([])), leaf)
                },
            },
        }
    }
}
