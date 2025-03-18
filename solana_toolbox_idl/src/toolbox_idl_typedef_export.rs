use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_typedef::ToolboxIdlTypedef;

impl ToolboxIdlTypedef {
    pub fn export(&self, backward_compatibility: bool) -> Value {
        if self.generics.is_empty() && !backward_compatibility {
            return self.type_flat.export(backward_compatibility);
        }
        let mut json_object = Map::new();
        if !self.generics.is_empty() {
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
            json_object
                .insert("generics".to_string(), Value::Array(json_generics));
        }
        if backward_compatibility {
            json_object.insert(
                "name".to_string(),
                Value::String(self.name.to_string()),
            );
        }
        json_object.insert(
            "type".to_string(),
            self.type_flat.export(backward_compatibility),
        );
        Value::Object(json_object)
    }
}
