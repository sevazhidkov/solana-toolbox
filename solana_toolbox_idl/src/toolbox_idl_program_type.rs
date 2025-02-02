use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_value_as_str_or_object_with_name_as_str_or_else;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramType {
    pub name: String,
    pub generics: Vec<String>,
    pub type_flat: ToolboxIdlTypeFlat,
}

impl ToolboxIdlProgramType {
    pub fn print(&self) {
        println!("----");
        if self.generics.is_empty() {
            println!("type.name: {}", self.name);
        } else {
            println!("type.name: {}<{}>", self.name, self.generics.join(","))
        }
        println!("type.type_flat: {}", self.type_flat.describe());
    }

    pub(crate) fn try_parse(
        idl_type_name: &str,
        idl_type: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramType, ToolboxIdlError> {
        let mut program_type_generics = vec![];
        if let Some(idl_type_generics) =
            idl_object_get_key_as_array(idl_type, "generics")
        {
            for (index, idl_type_generic) in
                idl_type_generics.iter().enumerate()
            {
                let idl_type_generic_name =
                    idl_value_as_str_or_object_with_name_as_str_or_else(
                        idl_type_generic,
                        &breadcrumbs.as_idl(&format!("[{}]", index)),
                    )?;
                program_type_generics.push(idl_type_generic_name.to_string());
            }
        }
        eprintln!(
            "program_type_generics:{}:{:?}",
            idl_type_name, program_type_generics
        );
        Ok(ToolboxIdlProgramType {
            name: idl_type_name.to_string(),
            generics: vec![],
            type_flat: ToolboxIdlTypeFlat::try_parse(
                &Value::Object(idl_type.clone()),
                breadcrumbs,
            )?,
        })
    }
}
