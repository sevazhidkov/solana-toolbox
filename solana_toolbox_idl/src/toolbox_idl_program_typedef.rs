use serde_json::Value;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_type_flat::ToolboxIdlProgramTypeFlat;
use crate::toolbox_idl_utils::idl_iter_get_scoped_values;
use crate::toolbox_idl_utils::idl_value_as_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_value_as_str_or_object_with_name_as_str_or_else;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramTypedef {
    pub name: String,
    pub generics: Vec<String>,
    pub type_flat: ToolboxIdlProgramTypeFlat,
}

impl ToolboxIdlProgramTypedef {
    pub fn try_parse(
        idl_typedef_name: &str,
        idl_typedef: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramTypedef, ToolboxIdlError> {
        let mut program_typedef_generics = vec![];
        if let Some(idl_typedef_generics) =
            idl_value_as_object_get_key_as_array(idl_typedef, "generics")
        {
            for (_, idl_typedef_generic, breadcrumbs) in
                idl_iter_get_scoped_values(idl_typedef_generics, breadcrumbs)?
            {
                let idl_typedef_generic_name =
                    idl_value_as_str_or_object_with_name_as_str_or_else(
                        idl_typedef_generic,
                        &breadcrumbs.idl(),
                    )?;
                program_typedef_generics
                    .push(idl_typedef_generic_name.to_string());
            }
        }
        Ok(ToolboxIdlProgramTypedef {
            name: idl_typedef_name.to_string(),
            generics: program_typedef_generics,
            type_flat: ToolboxIdlProgramTypeFlat::try_parse(
                idl_typedef,
                breadcrumbs,
            )?,
        })
    }
}
