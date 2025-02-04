use std::collections::HashMap;

use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_type::ToolboxIdlProgramType;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_utils::idl_as_bytes_or_else;

// TODO - should we simply expose the account API directly on this struct ?
#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramAccount {
    pub name: String,
    pub discriminator: Vec<u8>,
    pub data_type_flat: ToolboxIdlTypeFlat,
    pub data_type_full: ToolboxIdlTypeFull,
}

impl ToolboxIdlProgramAccount {
    pub fn print(&self) {
        println!("----");
        println!("account.name: {}", self.name);
        println!("account.discriminator: {:?}", self.discriminator);
        println!("account.data_type: {}", self.data_type_flat.describe());
    }

    pub(crate) fn try_parse(
        program_types: &HashMap<String, ToolboxIdlProgramType>,
        idl_account_name: &str,
        idl_account: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramAccount, ToolboxIdlError> {
        let program_account_discriminator =
            ToolboxIdlProgramAccount::try_parse_discriminator(
                idl_account_name,
                idl_account,
                &breadcrumbs.with_idl("discriminator"),
            )?;
        let program_account_data_type_flat =
            ToolboxIdlProgramAccount::try_parse_data_type_flat(
                idl_account_name,
                idl_account,
                breadcrumbs,
            )?;
        let program_account_data_type_full =
            ToolboxIdlProgramAccount::try_parse_data_type_full(
                program_types,
                &program_account_data_type_flat,
                breadcrumbs,
            )?;
        Ok(ToolboxIdlProgramAccount {
            name: idl_account_name.to_string(),
            discriminator: program_account_discriminator,
            data_type_flat: program_account_data_type_flat,
            data_type_full: program_account_data_type_full,
        })
    }

    fn try_parse_discriminator(
        idl_account_name: &str,
        idl_account: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Vec<u8>, ToolboxIdlError> {
        if let Some(idl_account_discriminator) =
            idl_account.get("discriminator")
        {
            return idl_as_bytes_or_else(
                idl_account_discriminator,
                &breadcrumbs.idl(),
            );
        }
        Ok(ToolboxIdl::compute_account_discriminator(idl_account_name))
    }

    fn try_parse_data_type_flat(
        idl_account_name: &str,
        idl_account: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFlat, ToolboxIdlError> {
        if idl_account.contains_key("type")
            || idl_account.contains_key("defined")
            || idl_account.contains_key("option")
            || idl_account.contains_key("vec")
            || idl_account.contains_key("array")
            || idl_account.contains_key("fields")
            || idl_account.contains_key("variants")
            || idl_account.contains_key("generic")
        {
            return ToolboxIdlTypeFlat::try_parse(
                &Value::Object(idl_account.clone()),
                breadcrumbs,
            );
        }
        Ok(ToolboxIdlTypeFlat::Defined {
            name: idl_account_name.to_string(),
            generics: vec![],
        })
    }

    fn try_parse_data_type_full(
        program_types: &HashMap<String, ToolboxIdlProgramType>,
        data_type_flat: &ToolboxIdlTypeFlat,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFull, ToolboxIdlError> {
        ToolboxIdlTypeFull::try_hydrate(
            program_types,
            &HashMap::new(),
            data_type_flat,
            breadcrumbs,
        )
    }
}
