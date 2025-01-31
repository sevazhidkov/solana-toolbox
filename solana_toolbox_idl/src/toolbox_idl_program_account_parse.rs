use std::collections::HashMap;

use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_account::ToolboxIdlProgramAccount;
use crate::toolbox_idl_program_type::ToolboxIdlProgramType;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_utils::idl_as_bytes_or_else;

impl ToolboxIdlProgramAccount {
    pub(crate) fn try_parse(
        program_types: &HashMap<String, ToolboxIdlProgramType>,
        idl_account_name: &str,
        idl_account: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramAccount, ToolboxIdlError> {
        let discriminator = ToolboxIdlProgramAccount::try_parse_discriminator(
            idl_account_name,
            idl_account,
            &breadcrumbs.with_idl("discriminator"),
        )?;
        let type_flat = ToolboxIdlProgramAccount::try_parse_type_flat(
            idl_account_name,
            idl_account,
            breadcrumbs,
        )?;
        let type_full = ToolboxIdlProgramAccount::try_parse_type_full(
            program_types,
            &type_flat,
            breadcrumbs,
        )?;
        Ok(ToolboxIdlProgramAccount {
            name: idl_account_name.to_string(),
            discriminator,
            type_flat,
            type_full,
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

    fn try_parse_type_flat(
        idl_account_name: &str,
        idl_account: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFlat, ToolboxIdlError> {
        if let Some(idl_account_type) = idl_account.get("type") {
            return ToolboxIdlTypeFlat::try_parse(
                idl_account_type,
                breadcrumbs,
            );
        }
        if idl_account.contains_key("fields") {
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

    fn try_parse_type_full(
        program_types: &HashMap<String, ToolboxIdlProgramType>,
        type_flat: &ToolboxIdlTypeFlat,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFull, ToolboxIdlError> {
        ToolboxIdlTypeFull::try_hydrate(
            program_types,
            &HashMap::new(),
            type_flat,
            breadcrumbs,
        )
    }
}
