use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_account::ToolboxIdlAccount;
use crate::toolbox_idl_format::ToolboxIdlFormat;

impl ToolboxIdlAccount {
    pub fn export(&self, format: &ToolboxIdlFormat) -> Value {
        let mut json_object = Map::new();
        if !format.use_object_for_unordered_named_array {
            json_object.insert("name".to_string(), json!(self.name));
        }
        if let Some(docs) = &self.docs {
            json_object.insert("docs".to_string(), json!(docs));
        }
        if let Some(space) = &self.space {
            json_object.insert("space".to_string(), json!(space));
        }
        json_object
            .insert("discriminator".to_string(), json!(self.discriminator));
        json_object
            .insert("type".to_string(), self.content_type_flat.export(format));
        json!(json_object)
    }
}
