use serde_json::{json, Value};

use crate::ToolboxIdlProgramInstructionAccount;

impl ToolboxIdlProgramInstructionAccount {
    pub fn as_json(&self, backward_compatibility: bool) -> Value {
        // TODO - implement all this
        json!({ "name": self.name })
    }
}
