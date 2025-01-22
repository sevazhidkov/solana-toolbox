use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_object_get_key_as_object_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_u64_or_else;

#[derive(Debug, Clone)]
pub struct ToolboxIdlLookupError {
    pub code: u64,
    pub name: String,
    pub msg: String,
}

impl ToolboxIdl {
    pub fn lookup_errors(
        &self
    ) -> Result<Vec<ToolboxIdlLookupError>, ToolboxIdlError> {
        let mut errors = vec![];
        for idl_error_name in self.errors.keys() {
            errors.push(self.lookup_error(idl_error_name)?);
        }
        Ok(errors)
    }

    pub fn lookup_error(
        &self,
        error_name: &str,
    ) -> Result<ToolboxIdlLookupError, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let idl_error_object = idl_object_get_key_as_object_or_else(
            &self.errors,
            error_name,
            &breadcrumbs.as_idl("errors"),
        )?;
        let idl_error_code = idl_object_get_key_as_u64_or_else(
            idl_error_object,
            "code",
            &breadcrumbs.as_idl(&format!("error[{}]", error_name)),
        )?;
        let idl_error_msg = idl_object_get_key_as_str_or_else(
            idl_error_object,
            "msg",
            &breadcrumbs.as_idl(&format!("error[{}]", error_name)),
        )?;
        Ok(ToolboxIdlLookupError {
            code: idl_error_code,
            name: error_name.to_string(),
            msg: idl_error_msg.to_string(),
        })
    }

    pub fn lookup_error_by_code(
        &self,
        error_code: u64,
    ) -> Result<ToolboxIdlLookupError, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        for (idl_error_name, idl_error) in self.errors.iter() {
            if let Some(idl_error_object) = idl_error.as_object() {
                if let Some(idl_error_code) = idl_error_object
                    .get("code")
                    .and_then(|idl_error_code| idl_error_code.as_u64())
                {
                    if idl_error_code == error_code {
                        return self.lookup_error(idl_error_name);
                    }
                }
            }
        }
        idl_err(
            "Could not find error",
            &breadcrumbs.as_idl(&format!("error({})", error_code)),
        )
    }
}

impl ToolboxIdlLookupError {
    pub fn print(&self) {
        println!("----");
        println!("error.code: {:?}", self.code);
        println!("error.name: {:?}", self.name);
        println!("error.msg: {:?}", self.msg);
    }
}
