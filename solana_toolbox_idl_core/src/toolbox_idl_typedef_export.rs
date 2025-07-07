use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_format::ToolboxIdlFormat;
use crate::toolbox_idl_typedef::ToolboxIdlTypedef;

impl ToolboxIdlTypedef {
    pub fn export(&self, format: &ToolboxIdlFormat) -> Value {
        if format.can_skip_typedef_type_object_wrap
            && self.docs.is_none()
            && self.serialization.is_none()
            && self.repr.is_none()
            && self.generics.is_empty()
        {
            return self.type_flat.export(format);
        }
        let mut json_object = Map::new();
        if !format.use_object_for_unordered_named_array {
            json_object.insert("name".to_string(), json!(self.name));
        }
        if let Some(docs) = &self.docs {
            json_object.insert("docs".to_string(), json!(docs));
        }
        if let Some(serialization) = &self.serialization {
            json_object
                .insert("serialization".to_string(), json!(serialization));
        }
        if let Some(repr) = &self.repr {
            json_object.insert(
                "repr".to_string(),
                json!({
                    "kind": repr,
                }),
            );
        }
        if !self.generics.is_empty() {
            let mut json_generics = vec![];
            for generic in &self.generics {
                if format.can_skip_typedef_generic_kind_key {
                    json_generics.push(json!(generic));
                } else {
                    json_generics.push(json!({
                        "kind": "type",
                        "name": generic
                    }));
                }
            }
            json_object.insert("generics".to_string(), json!(json_generics));
        }
        json_object.insert("type".to_string(), self.type_flat.export(format));
        json!(json_object)
    }
}
