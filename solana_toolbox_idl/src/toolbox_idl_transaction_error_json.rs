use serde_json::json;
use serde_json::Value;

use crate::toolbox_idl_transaction_error::ToolboxIdlTransactionError;

impl ToolboxIdlTransactionError {
    pub fn as_json(&self, backward_compatibility: bool) -> Value {
        if backward_compatibility {
            json!({
                "name": self.name,
                "code": self.code,
                "msg": self.msg
            })
        } else if self.msg.is_empty() {
            json!({
                "code": self.code,
                "msg": self.msg
            })
        } else {
            json!(self.code)
        }
    }
}
