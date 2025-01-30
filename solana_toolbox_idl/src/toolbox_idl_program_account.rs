use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_typedef::ToolboxIdlProgramTypedef;
use crate::toolbox_idl_utils::idl_as_bytes_or_else;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramAccount {
    pub name: String,
    pub discriminator: Vec<u8>,
    pub typedef: ToolboxIdlProgramTypedef,
}

impl ToolboxIdlProgramAccount {
    pub fn print(&self) {
        println!("----");
        println!("account.name: {}", self.name);
        println!("account.discriminator: {:?}", self.discriminator);
        println!("account.typedef: {}", self.typedef.describe());
    }

    pub(crate) fn try_parse(
        idl_account_name: &str,
        idl_account_object: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramAccount, ToolboxIdlError> {
        Ok(ToolboxIdlProgramAccount {
            name: idl_account_name.to_string(),
            discriminator: ToolboxIdlProgramAccount::try_parse_discriminator(
                idl_account_name,
                idl_account_object,
                &breadcrumbs.with_idl("discriminator"),
            )?,
            typedef: ToolboxIdlProgramAccount::try_parse_typedef(
                idl_account_name,
                idl_account_object,
                &breadcrumbs.with_idl("typedef"),
            )?,
        })
    }

    fn try_parse_discriminator(
        idl_account_name: &str,
        idl_account_object: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Vec<u8>, ToolboxIdlError> {
        if let Some(idl_account_discriminator) =
            idl_account_object.get("discriminator")
        {
            return idl_as_bytes_or_else(
                idl_account_discriminator,
                &breadcrumbs.idl(),
            );
        }
        Ok(ToolboxIdl::compute_account_discriminator(idl_account_name))
    }

    fn try_parse_typedef(
        idl_account_name: &str,
        idl_account_object: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramTypedef, ToolboxIdlError> {
        if let Some(idl_account_typedef) = idl_account_object.get("type") {
            return ToolboxIdlProgramTypedef::try_parse(
                idl_account_typedef,
                breadcrumbs,
            );
        }
        if idl_account_object.contains_key("fields") {
            return ToolboxIdlProgramTypedef::try_parse(
                &Value::Object(idl_account_object.clone()),
                breadcrumbs,
            );
        }
        Ok(ToolboxIdlProgramTypedef::Defined {
            name: idl_account_name.to_string(),
            generics: vec![],
        })
    }
}
