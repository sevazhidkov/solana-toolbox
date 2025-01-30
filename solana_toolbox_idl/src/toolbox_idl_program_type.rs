use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::{
    toolbox_idl_program_def::ToolboxIdlProgramDef,
    toolbox_idl_utils::{
        idl_object_get_key_as_array,
        idl_value_as_str_or_object_with_name_as_str_or_else,
    },
    ToolboxIdlBreadcrumbs, ToolboxIdlError,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramType {
    pub name: String,
    pub generics: HashMap<String, usize>,
    pub def: ToolboxIdlProgramDef,
}

impl ToolboxIdlProgramType {
    pub fn print(&self) {
        println!("----");
        println!("type.name: {}", self.name);
        for (key, value) in &self.generics {
            println!("type.generic: {}: {}", key, value);
        }
        println!("type.def: {}", self.def.describe());
    }

    pub(crate) fn try_parse(
        idl_type_name: &str,
        idl_type_object: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramType, ToolboxIdlError> {
        if let Some(idl_type_type) = idl_type_object.get("type") {
            let mut program_type_generics = HashMap::new();
            // TODO - support generics
            if let Some(idl_type_generics) =
                idl_object_get_key_as_array(idl_type_object, "generics")
            {
                for (index, idl_type_generic) in
                    idl_type_generics.iter().enumerate()
                {
                    let idl_type_generic_name =
                        idl_value_as_str_or_object_with_name_as_str_or_else(
                            idl_type_generic,
                            &breadcrumbs.as_idl(&format!("[{}]", index)),
                        )?;
                    program_type_generics
                        .insert(idl_type_generic_name.to_string(), index);
                }
            }
            Ok(ToolboxIdlProgramType {
                name: idl_type_name.to_string(),
                generics: program_type_generics,
                def: ToolboxIdlProgramDef::try_parse(
                    idl_type_type,
                    &breadcrumbs.with_idl("def"),
                )?,
            })
        } else {
            Ok(ToolboxIdlProgramType {
                name: idl_type_name.to_string(),
                generics: HashMap::new(),
                def: ToolboxIdlProgramDef::try_parse(
                    &Value::Object(idl_type_object.clone()),
                    &breadcrumbs.with_idl("def"),
                )?,
            })
        }
    }
}
