use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_transaction_error::ToolboxIdlTransactionError;

impl ToolboxIdlTransactionError {
    pub fn export(&self, backward_compatibility: bool) -> Value {
        if !backward_compatibility && self.msg.is_empty() {
            return json!(self.code);
        }
        let mut json_object = Map::new();
        if backward_compatibility {
            json_object.insert("name".to_string(), json!(self.name));
        }
        json_object.insert("code".to_string(), json!(self.code));
        json_object.insert("msg".to_string(), json!(self.msg));
        json!(json_object)
    }
}
