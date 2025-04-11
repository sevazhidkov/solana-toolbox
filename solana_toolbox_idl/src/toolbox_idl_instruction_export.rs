use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_format::ToolboxIdlFormat;
use crate::toolbox_idl_instruction::ToolboxIdlInstruction;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_utils::idl_convert_to_camel_name;

impl ToolboxIdlInstruction {
    pub fn export(&self, format: &ToolboxIdlFormat) -> Value {
        let mut json_object = Map::new();
        if !format.use_object_for_unordered_named_array {
            json_object.insert("name".to_string(), self.export_name(format));
        }
        if let Some(docs) = &self.docs {
            json_object.insert("docs".to_string(), json!(docs));
        }
        json_object
            .insert("discriminator".to_string(), json!(self.discriminator));
        let mut json_accounts = vec![];
        for account in &self.accounts {
            json_accounts.push(account.export(format));
        }
        json_object.insert("accounts".to_string(), json!(json_accounts));
        json_object.insert(
            "args".to_string(),
            self.args_type_flat_fields.export(format),
        );
        if self.return_type_flat != ToolboxIdlTypeFlat::nothing() {
            json_object.insert(
                "returns".to_string(),
                self.return_type_flat.export(format),
            );
        }
        json!(json_object)
    }

    fn export_name(&self, format: &ToolboxIdlFormat) -> Value {
        if format.use_camel_case_instruction_names {
            json!(idl_convert_to_camel_name(&self.name))
        } else {
            json!(self.name)
        }
    }
}
