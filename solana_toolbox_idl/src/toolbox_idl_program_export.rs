use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_program::ToolboxIdlProgram;

impl ToolboxIdlProgram {
    pub fn export(&self, backward_compatibility: bool) -> Value {
        let mut json_program = Map::new();
        if let Some(name) = &self.name {
            json_program.insert("name".to_string(), json!(name));
        }
        json_program.insert(
            "instructions".to_string(),
            self.export_instructions(backward_compatibility),
        );
        json_program.insert(
            "accounts".to_string(),
            self.export_accounts(backward_compatibility),
        );
        json_program.insert(
            "errors".to_string(),
            self.export_errors(backward_compatibility),
        );
        json_program.insert(
            "types".to_string(),
            self.export_typedefs(backward_compatibility),
        );
        json!(json_program)
    }

    fn export_instructions(&self, backward_compatibility: bool) -> Value {
        if backward_compatibility {
            let mut json_instructions = vec![];
            for program_instruction in self.instructions.values() {
                json_instructions
                    .push(program_instruction.export(backward_compatibility));
            }
            json!(json_instructions)
        } else {
            let mut json_instructions = Map::new();
            for program_instruction in self.instructions.values() {
                json_instructions.insert(
                    program_instruction.name.to_string(),
                    program_instruction.export(backward_compatibility),
                );
            }
            json!(json_instructions)
        }
    }

    fn export_accounts(&self, backward_compatibility: bool) -> Value {
        if backward_compatibility {
            let mut json_accounts = vec![];
            for program_account in self.accounts.values() {
                json_accounts
                    .push(program_account.export(backward_compatibility));
            }
            json!(json_accounts)
        } else {
            let mut json_accounts = Map::new();
            for program_account in self.accounts.values() {
                json_accounts.insert(
                    program_account.name.to_string(),
                    program_account.export(backward_compatibility),
                );
            }
            json!(json_accounts)
        }
    }

    fn export_errors(&self, backward_compatibility: bool) -> Value {
        if backward_compatibility {
            let mut json_errors = vec![];
            for program_error in self.errors.values() {
                json_errors.push(program_error.export(backward_compatibility));
            }
            json!(json_errors)
        } else {
            let mut json_errors = Map::new();
            for program_error in self.errors.values() {
                json_errors.insert(
                    program_error.name.to_string(),
                    program_error.export(backward_compatibility),
                );
            }
            json!(json_errors)
        }
    }

    fn export_typedefs(&self, backward_compatibility: bool) -> Value {
        if backward_compatibility {
            let mut json_typedefs = vec![];
            for program_typedef in self.typedefs.values() {
                json_typedefs
                    .push(program_typedef.export(backward_compatibility));
            }
            json!(json_typedefs)
        } else {
            let mut json_typedefs = Map::new();
            for program_typedef in self.typedefs.values() {
                json_typedefs.insert(
                    program_typedef.name.to_string(),
                    program_typedef.export(backward_compatibility),
                );
            }
            json!(json_typedefs)
        }
    }
}
