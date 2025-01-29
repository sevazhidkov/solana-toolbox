use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_u64_or_else;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramError {
    pub code: u64,
    pub name: String,
    pub msg: String,
}

impl ToolboxIdlProgramError {
    pub fn print(&self) {
        println!("----");
        println!("error.code: {}", self.code);
        println!("error.name: {}", self.name);
        println!("error.msg: {}", self.msg);
    }

    pub(crate) fn try_parse(
        idl_error_name: &str,
        idl_error_object: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramError, ToolboxIdlError> {
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
        Ok(ToolboxIdlProgramError {
            code: idl_error_code,
            name: idl_error_name.to_string(),
            msg: idl_error_msg.to_string(),
        })
    }
}
