use serde_json::json;
use serde_json::Value;

use crate::toolbox_idl_typedef::ToolboxIdlTypedef;

impl ToolboxIdlTypedef {
    pub fn as_json(&self, backward_compatibility: bool) -> Value {
        let mut json_generics = vec![];
        for generic in &self.generics {
            if backward_compatibility {
                json_generics.push(json!({
                    "kind": "type",
                    "name": generic
                }));
            } else {
                json_generics.push(json!(generic));
            }
        }
        if json_generics.is_empty() {
            if backward_compatibility {
                json!({
                    "name": self.name,
                    "type": self.type_flat.as_json(backward_compatibility)
                })
            } else {
                self.type_flat.as_json(backward_compatibility)
            }
        } else {
            if backward_compatibility {
                json!({
                    "name": self.name,
                    "type": self.type_flat.as_json(backward_compatibility),
                    "generics": json_generics,
                })
            } else {
                json!({
                    "type": self.type_flat.as_json(backward_compatibility),
                    "generics": json_generics,
                })
            }
        }
    }
}
