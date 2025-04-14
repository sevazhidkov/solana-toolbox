use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_format::ToolboxIdlFormat;
use crate::toolbox_idl_program::ToolboxIdlProgram;

impl ToolboxIdlProgram {
    pub fn export(&self, format: &ToolboxIdlFormat) -> Value {
        let mut json_program = Map::new();
        if format.use_root_also_as_metadata_object {
            self.export_metadata_to(&mut json_program);
        }
        let mut json_program_metadata = Map::new();
        self.export_metadata_to(&mut json_program_metadata);
        json_program
            .insert("metadata".to_string(), json!(json_program_metadata));
        json_program.insert("types".to_string(), self.export_typedefs(format));
        json_program
            .insert("accounts".to_string(), self.export_accounts(format));
        json_program.insert(
            "instructions".to_string(),
            self.export_instructions(format),
        );
        json_program.insert("events".to_string(), self.export_events(format));
        json_program.insert("errors".to_string(), self.export_errors(format));
        json!(json_program)
    }

    fn export_metadata_to(&self, json_object: &mut Map<String, Value>) {
        if let Some(address) = &self.metadata.address {
            json_object
                .insert("address".to_string(), json!(address.to_string()));
        }
        if let Some(name) = &self.metadata.name {
            json_object.insert("name".to_string(), json!(name));
        }
        if let Some(description) = &self.metadata.description {
            json_object.insert("description".to_string(), json!(description));
        }
        if let Some(docs) = &self.metadata.docs {
            json_object.insert("docs".to_string(), json!(docs));
        }
        if let Some(version) = &self.metadata.version {
            json_object.insert("version".to_string(), json!(version));
        }
        if let Some(spec) = &self.metadata.spec {
            json_object.insert("spec".to_string(), json!(spec));
        }
    }

    fn export_typedefs(&self, format: &ToolboxIdlFormat) -> Value {
        if format.use_object_for_unordered_named_array {
            let mut json_typedefs = Map::new();
            for program_typedef in self.typedefs.values() {
                json_typedefs.insert(
                    program_typedef.name.to_string(),
                    program_typedef.export(format),
                );
            }
            return json!(json_typedefs);
        }
        let mut json_typedefs = vec![];
        for program_typedef in self.typedefs.values() {
            json_typedefs.push(program_typedef.export(format));
        }
        json!(json_typedefs)
    }

    fn export_accounts(&self, format: &ToolboxIdlFormat) -> Value {
        if format.use_object_for_unordered_named_array {
            let mut json_accounts = Map::new();
            for program_account in self.accounts.values() {
                json_accounts.insert(
                    program_account.name.to_string(),
                    program_account.export(format),
                );
            }
            return json!(json_accounts);
        }
        let mut json_accounts = vec![];
        for program_account in self.accounts.values() {
            json_accounts.push(program_account.export(format));
        }
        json!(json_accounts)
    }

    fn export_instructions(&self, format: &ToolboxIdlFormat) -> Value {
        if format.use_object_for_unordered_named_array {
            let mut json_instructions = Map::new();
            for program_instruction in self.instructions.values() {
                json_instructions.insert(
                    program_instruction.name.to_string(),
                    program_instruction.export(format),
                );
            }
            return json!(json_instructions);
        }
        let mut json_instructions = vec![];
        for program_instruction in self.instructions.values() {
            json_instructions.push(program_instruction.export(format));
        }
        json!(json_instructions)
    }

    fn export_events(&self, format: &ToolboxIdlFormat) -> Value {
        if format.use_object_for_unordered_named_array {
            let mut json_events = Map::new();
            for program_event in self.events.values() {
                json_events.insert(
                    program_event.name.to_string(),
                    program_event.export(format),
                );
            }
            return json!(json_events);
        }
        let mut json_events = vec![];
        for program_event in self.events.values() {
            json_events.push(program_event.export(format));
        }
        json!(json_events)
    }

    fn export_errors(&self, format: &ToolboxIdlFormat) -> Value {
        if format.use_object_for_unordered_named_array {
            let mut json_errors = Map::new();
            for program_error in self.errors.values() {
                json_errors.insert(
                    program_error.name.to_string(),
                    program_error.export(format),
                );
            }
            return json!(json_errors);
        }
        let mut json_errors = vec![];
        for program_error in self.errors.values() {
            json_errors.push(program_error.export(format));
        }
        json!(json_errors)
    }
}
