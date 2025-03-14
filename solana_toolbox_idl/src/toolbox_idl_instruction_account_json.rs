use serde_json::json;
use serde_json::Value;

use crate::ToolboxIdlInstructionAccount;

impl ToolboxIdlInstructionAccount {
    pub fn as_json(&self, backward_compatibility: bool) -> Value {
        // TODO - implement all this json
        json!({ "name": self.name })
    }
}
