use anyhow::anyhow;
use anyhow::Result;
use serde_json::Value;

use crate::toolbox_idl_path::ToolboxIdlPath;

impl ToolboxIdlPath {
    pub fn try_extract_json_value(&self, value: &Value) -> Result<Value> {
        let Some((current, next)) = self.split_first() else {
            return Ok(value.clone());
        };
        match value {
            Value::Null => {
                Err(anyhow!("Null value does not contain field: {}", current))
            },
            Value::Bool(_) => {
                Err(anyhow!("Bool value does not contain field: {}", current))
            },
            Value::Number(_) => {
                Err(anyhow!("Number value does not contain field: {}", current))
            },
            Value::String(_) => {
                Err(anyhow!("String value does not contain field: {}", current))
            },
            Value::Array(values) => {
                let length = values.len();
                let index = current.parse::<usize>()?;
                if index >= length {
                    return Err(anyhow!(
                        "Invalid array index: {} (length: {})",
                        index,
                        length
                    ));
                }
                next.try_extract_json_value(&values[index])
            },
            Value::Object(map) => {
                for (key, value) in map {
                    if key == &current {
                        return next.try_extract_json_value(value);
                    }
                }
                Err(anyhow!("Could not find object key: {}", current))
            },
        }
    }
}
