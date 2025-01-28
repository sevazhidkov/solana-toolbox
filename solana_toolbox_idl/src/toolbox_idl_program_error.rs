use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_u64_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_scoped_named_object_array_or_else;
use serde_json::Map;
use std::collections::HashMap;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramError {
    pub name: String,
    pub msg: String,
}

pub(crate) fn idl_program_errors_parse(
    idl_root_object: &Map<String, Value>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<HashMap<u64, ToolboxIdlProgramError>, ToolboxIdlError> {
    let mut program_errors = HashMap::new();
    for (idl_error_name, idl_error_object, breadcrumbs) in
        idl_object_get_key_as_scoped_named_object_array_or_else(
            idl_root_object,
            "errors",
            breadcrumbs,
        )?
    {
        let idl_error_code = idl_object_get_key_as_u64_or_else(
            idl_error_object,
            "code",
            &breadcrumbs.idl(),
        )?;
        let idl_error_msg = idl_object_get_key_as_str_or_else(
            idl_error_object,
            "msg",
            &breadcrumbs.idl(),
        )?;
        program_errors
            .insert(idl_error_code, ToolboxIdlProgramError {
                name: idl_error_name.to_string(),
                msg: idl_error_msg.to_string(),
            });
    }
    Ok(program_errors)
}
