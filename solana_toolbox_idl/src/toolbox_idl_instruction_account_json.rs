use serde_json::json;
use serde_json::Value;

use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccount;

impl ToolboxIdlInstructionAccount {
    pub fn as_json(&self, backward_compatibility: bool) -> Value {
        // TODO - implement all this json
        json!({ "name": self.name })
    }
}
