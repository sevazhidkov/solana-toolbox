use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_instruction::ToolboxIdlInstruction;

impl ToolboxIdlInstruction {
    pub fn export(&self, backward_compatibility: bool) -> Value {
        let mut json_object = Map::new();
        if backward_compatibility {
            json_object.insert("name".to_string(), json!(self.name));
        }
        if let Some(docs) = &self.docs {
            json_object.insert("docs".to_string(), json!(docs));
        }
        json_object
            .insert("discriminator".to_string(), json!(self.discriminator));
        let mut json_accounts = vec![];
        for account in &self.accounts {
            json_accounts.push(account.export(backward_compatibility));
        }
        json_object.insert("accounts".to_string(), json!(json_accounts));
        json_object.insert(
            "args".to_string(),
            self.args_type_flat_fields.export(backward_compatibility),
        );
        json!(json_object)
    }
}
