use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_format::ToolboxIdlFormat;

impl ToolboxIdlError {
    pub fn export(&self, format: &ToolboxIdlFormat) -> Value {
        if format.can_shortcut_error_to_number_if_no_msg
            && format.use_object_for_unordered_named_array
            && self.msg.is_none()
        {
            return json!(self.code);
        }
        let mut json_object = Map::new();
        if !format.use_object_for_unordered_named_array {
            json_object.insert("name".to_string(), json!(self.name));
        }
        json_object.insert("code".to_string(), json!(self.code));
        if let Some(msg) = &self.msg {
            json_object.insert("msg".to_string(), json!(msg));
        }
        json!(json_object)
    }
}
