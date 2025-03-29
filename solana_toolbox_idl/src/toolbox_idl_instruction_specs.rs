use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_instruction::ToolboxIdlInstruction;

impl ToolboxIdlInstruction {
    pub fn get_specs(&self) -> (Value, Map<String, Value>) {
        let instruction_specs_payload = self.args_type_full_fields.explain();
        let mut instruction_specs_addresses = Map::new();
        for account in &self.accounts {
            if let Some(account_address) = &account.address {
                instruction_specs_addresses.insert(
                    account.name.to_string(),
                    json!(account_address.to_string()),
                );
            } else if let Some(account_pda) = &account.pda {
                let mut specs_blobs = vec![];
                for account_pda_seed in &account_pda.seeds {
                    if let Some((kind, path)) = account_pda_seed.info() {
                        specs_blobs.push(json!({ kind: path }));
                    }
                }
                if let Some(account_pda_program) = &account_pda.program {
                    if let Some((kind, path)) = account_pda_program.info() {
                        specs_blobs.push(json!({ kind: path }));
                    }
                }
                instruction_specs_addresses
                    .insert(account.name.to_string(), json!(specs_blobs));
            } else {
                instruction_specs_addresses
                    .insert(account.name.to_string(), json!(null));
            }
        }
        (instruction_specs_payload, instruction_specs_addresses)
    }
}
