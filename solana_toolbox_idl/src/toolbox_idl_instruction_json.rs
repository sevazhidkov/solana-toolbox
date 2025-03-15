use serde_json::json;
use serde_json::Value;

use crate::toolbox_idl_instruction::ToolboxIdlInstruction;

impl ToolboxIdlInstruction {
    pub fn as_json(&self, backward_compatibility: bool) -> Value {
        let mut json_accounts = vec![];
        for account in &self.accounts {
            json_accounts.push(account.as_json(backward_compatibility));
        }
        if backward_compatibility {
            json!({
                "name": self.name,
                "discriminator": self.discriminator,
                "accounts": json_accounts,
                "args": self.args_type_flat_fields.as_json(backward_compatibility)
            })
        } else {
            json!({
                "discriminator": self.discriminator,
                "accounts": json_accounts,
                "args": self.args_type_flat_fields.as_json(backward_compatibility)
            })
        }
    }
}
