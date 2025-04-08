use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_instruction::ToolboxIdlInstruction;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPdaBlob;

impl ToolboxIdlInstruction {
    pub fn get_needs(&self) -> (Value, Map<String, Value>) {
        let instruction_needs_payload = self.args_type_full_fields.explain();
        let mut instruction_needs_addresses = Map::new();
        for account in &self.accounts {
            if let Some(account_address) = &account.address {
                instruction_needs_addresses.insert(
                    account.name.to_string(),
                    json!(account_address.to_string()),
                );
            } else if let Some(account_pda) = &account.pda {
                let mut needs_blobs = vec![];
                for account_pda_seed in &account_pda.seeds {
                    if let Some((key, value)) = get_need(account_pda_seed) {
                        needs_blobs.push(json!({ key: value }));
                    }
                }
                if let Some(account_pda_program) = &account_pda.program {
                    if let Some((key, value)) = get_need(account_pda_program) {
                        needs_blobs.push(json!({ key: value }));
                    }
                }
                instruction_needs_addresses
                    .insert(account.name.to_string(), json!(needs_blobs));
            } else {
                instruction_needs_addresses
                    .insert(account.name.to_string(), json!(null));
            }
        }
        (instruction_needs_payload, instruction_needs_addresses)
    }
}

fn get_need(
    blob: &ToolboxIdlInstructionAccountPdaBlob,
) -> Option<(&str, String)> {
    match blob {
        ToolboxIdlInstructionAccountPdaBlob::Const { .. } => None,
        ToolboxIdlInstructionAccountPdaBlob::Arg { path, .. } => {
            Some(("arg", path.export()))
        },
        ToolboxIdlInstructionAccountPdaBlob::Account { path, .. } => {
            Some(("account", path.export()))
        },
    }
}
