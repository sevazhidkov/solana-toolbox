use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_event::ToolboxIdlEvent;
use crate::toolbox_idl_format::ToolboxIdlFormat;

impl ToolboxIdlEvent {
    pub fn export(&self, format: &ToolboxIdlFormat) -> Value {
        let mut json_object = Map::new();
        if !format.use_object_for_unordered_named_array {
            json_object.insert("name".to_string(), json!(self.name));
        }
        if let Some(docs) = &self.docs {
            json_object.insert("docs".to_string(), json!(docs));
        }
        json_object
            .insert("discriminator".to_string(), json!(self.discriminator));
        json_object
            .insert("type".to_string(), self.info_type_flat.export(format));
        json!(json_object)
    }
}
