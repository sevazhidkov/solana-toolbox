use std::collections::HashMap;

use serde_json::Map;
use serde_json::Value;
use sha2::Digest;
use sha2::Sha256;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_type_flat::ToolboxIdlProgramTypeFlat;
use crate::toolbox_idl_program_type_full::ToolboxIdlProgramTypeFull;
use crate::toolbox_idl_program_typedef::ToolboxIdlProgramTypedef;
use crate::toolbox_idl_utils::idl_as_bytes_or_else;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;

// TODO - should we simply expose the account API directly on this struct ?
#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramAccount {
    pub name: String,
    pub discriminator: Vec<u8>,
    pub data_type_flat: ToolboxIdlProgramTypeFlat,
    pub data_type_full: ToolboxIdlProgramTypeFull,
}

impl ToolboxIdlProgramAccount {
    pub fn try_parse(
        program_typedefs: &HashMap<String, ToolboxIdlProgramTypedef>,
        idl_account_name: &str,
        idl_account: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramAccount, ToolboxIdlError> {
        let idl_account =
            idl_as_object_or_else(idl_account, &breadcrumbs.idl())?;
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
                program_typedefs,
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
            idl_object_get_key_as_array(idl_account, "discriminator")
        {
            return idl_as_bytes_or_else(
                idl_account_discriminator,
                &breadcrumbs.idl(),
            );
        }
        let mut hasher = Sha256::new();
        hasher.update(format!("account:{}", idl_account_name));
        Ok(hasher.finalize()[..8].to_vec())
    }

    fn try_parse_data_type_flat(
        idl_account_name: &str,
        idl_account: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramTypeFlat, ToolboxIdlError> {
        if idl_account.contains_key("type")
            || idl_account.contains_key("defined")
            || idl_account.contains_key("option")
            || idl_account.contains_key("vec")
            || idl_account.contains_key("array")
            || idl_account.contains_key("fields")
            || idl_account.contains_key("variants")
            || idl_account.contains_key("generic")
        {
            return ToolboxIdlProgramTypeFlat::try_parse(
                &Value::Object(idl_account.clone()),
                breadcrumbs,
            );
        }
        Ok(ToolboxIdlProgramTypeFlat::Defined {
            name: idl_account_name.to_string(),
            generics: vec![],
        })
    }

    fn try_parse_data_type_full(
        program_typedefs: &HashMap<String, ToolboxIdlProgramTypedef>,
        data_type_flat: &ToolboxIdlProgramTypeFlat,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramTypeFull, ToolboxIdlError> {
        ToolboxIdlProgramTypeFull::try_hydrate(
            program_typedefs,
            &HashMap::new(),
            data_type_flat,
            breadcrumbs,
        )
    }
}
