use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_def::ToolboxIdlProgramDef;
use crate::toolbox_idl_utils::idl_as_bytes_or_else;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramAccount {
    pub name: String,
    pub discriminator: Vec<u8>,
    pub def: ToolboxIdlProgramDef,
}

impl ToolboxIdlProgramAccount {
    pub fn print(&self) {
        println!("----");
        println!("account.name: {}", self.name);
        println!("account.discriminator: {:?}", self.discriminator);
        println!("account.def: {}", self.def.describe());
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
            def: ToolboxIdlProgramAccount::try_parse_def(
                idl_account_name,
                idl_account_object,
                &breadcrumbs.with_idl("def"),
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

    fn try_parse_def(
        idl_account_name: &str,
        idl_account_object: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramDef, ToolboxIdlError> {
        if let Some(idl_account_def) = idl_account_object.get("type") {
            return ToolboxIdlProgramDef::try_parse(
                idl_account_def,
                breadcrumbs,
            );
        }
        if idl_account_object.contains_key("fields") {
            return ToolboxIdlProgramDef::try_parse(
                &Value::Object(idl_account_object.clone()),
                breadcrumbs,
            );
        }
        Ok(ToolboxIdlProgramDef::Defined {
            name: idl_account_name.to_string(),
            generics: vec![],
        })
    }
}
