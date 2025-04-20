use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use serde_json::Value;

use crate::toolbox_idl_path::ToolboxIdlPath;

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
                self.to_string()
            )),
            Value::Bool(_) => Err(anyhow!(
                "Bool value does not contain path: {}",
                self.to_string()
            )),
            Value::Number(_) => Err(anyhow!(
                "Number value does not contain path: {}",
                self.to_string()
            )),
            Value::String(_) => Err(anyhow!(
                "String value does not contain path: {}",
                self.to_string()
            )),
            Value::Array(values) => {
                let length = values.len();
                let index =
                    usize::try_from(current.code().context("Array index")?)?;
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
                let key = current.to_string();
                for (object_key, object_value) in map {
                    if object_key == &key {
                        return next.try_get_json_value(object_value);
                    }
                }
                Err(anyhow!("Could not find object key: {}", key))
            },
        }
    }
}
