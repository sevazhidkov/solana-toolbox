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
        if !self.blobs.is_empty() {
            let mut json_blobs = vec![];
            for blob in &self.blobs {
                json_blobs.push(json!({
                    "offset": blob.0,
                    "value": blob.1,
                }));
            }
            json_object.insert("blobs".to_string(), json!(json_blobs));
        }
        json_object
            .insert("discriminator".to_string(), json!(self.discriminator));
        json_object
            .insert("type".to_string(), self.content_type_flat.export(format));
        json!(json_object)
    }
}
