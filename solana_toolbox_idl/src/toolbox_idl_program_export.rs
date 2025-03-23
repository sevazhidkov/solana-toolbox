use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_program::ToolboxIdlProgram;

impl ToolboxIdlProgram {
    pub fn export(&self, backward_compatibility: bool) -> Value {
        if backward_compatibility {
            let mut json_instructions = vec![];
            for program_instruction in self.instructions.values() {
                json_instructions
                    .push(program_instruction.export(backward_compatibility));
            }
            let mut json_accounts = vec![];
            for program_account in self.accounts.values() {
                json_accounts
                    .push(program_account.export(backward_compatibility));
            }
            let mut json_errors = vec![];
            for program_error in self.errors.values() {
                json_errors.push(program_error.export(backward_compatibility));
            }
            let mut json_typedefs = vec![];
            for program_typedef in self.typedefs.values() {
                json_typedefs
                    .push(program_typedef.export(backward_compatibility));
            }
            json!({
                "instructions": json_instructions,
                "accounts": json_accounts,
                "errors": json_errors,
                "types": json_typedefs,
            })
        } else {
            let mut json_instructions = Map::new();
            for program_instruction in self.instructions.values() {
                json_instructions.insert(
                    program_instruction.name.to_string(),
                    program_instruction.export(backward_compatibility),
                );
            }
            let mut json_accounts = Map::new();
            for program_account in self.accounts.values() {
                json_accounts.insert(
                    program_account.name.to_string(),
                    program_account.export(backward_compatibility),
                );
            }
            let mut json_errors = Map::new();
            for program_error in self.errors.values() {
                json_errors.insert(
                    program_error.name.to_string(),
                    program_error.export(backward_compatibility),
                );
            }
            let mut json_typedefs = Map::new();
            for program_typedef in self.typedefs.values() {
                json_typedefs.insert(
                    program_typedef.name.to_string(),
                    program_typedef.export(backward_compatibility),
                );
            }
            json!({
                "instructions": json_instructions,
                "accounts": json_accounts,
                "errors": json_errors,
                "types": json_typedefs,
            })
        }
    }
}
