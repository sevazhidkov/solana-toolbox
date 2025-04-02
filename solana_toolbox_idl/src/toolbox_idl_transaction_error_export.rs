use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_info_format::ToolboxIdlInfoFormat;
use crate::toolbox_idl_transaction_error::ToolboxIdlTransactionError;

impl ToolboxIdlTransactionError {
    pub fn export(&self, format: &ToolboxIdlInfoFormat) -> Value {
        if self.msg.is_empty()
            && format.can_shortcut_error_to_number_if_no_msg()
        {
            return json!(self.code);
        }
        let mut json_object = Map::new();
        if !format.use_object_for_unordered_named_array() {
            json_object.insert("name".to_string(), json!(self.name));
        }
        json_object.insert("code".to_string(), json!(self.code));
        json_object.insert("msg".to_string(), json!(self.msg));
        json!(json_object)
    }
}
