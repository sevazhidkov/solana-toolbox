use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_account::ToolboxIdlAccount;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_typedef::ToolboxIdlTypedef;
use crate::toolbox_idl_utils::idl_as_bytes_or_else;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_convert_to_type_name;
use crate::toolbox_idl_utils::idl_hash_discriminator_from_string;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;

impl ToolboxIdlAccount {
    pub fn try_parse(
        idl_account_name: &str,
        idl_account: &Value,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlAccount, ToolboxIdlError> {
        let idl_account =
            idl_as_object_or_else(idl_account, &breadcrumbs.idl())?;
        let account_discriminator = ToolboxIdlAccount::try_parse_discriminator(
            idl_account_name,
            idl_account,
            &breadcrumbs.with_idl("discriminator"),
        )?;
        let account_docs = idl_account.get("docs").cloned();
        let account_data_type_flat =
            ToolboxIdlAccount::try_parse_data_type_flat(
                idl_account_name,
                idl_account,
                breadcrumbs,
            )?;
        let account_data_type_full = account_data_type_flat.try_hydrate(
            &HashMap::new(),
            typedefs,
            breadcrumbs,
        )?;
        Ok(ToolboxIdlAccount {
            name: idl_account_name.to_string(),
            docs: account_docs,
            discriminator: account_discriminator,
            content_type_flat: account_data_type_flat,
            content_type_full: account_data_type_full.into(),
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
        Ok(idl_hash_discriminator_from_string(&format!(
            "account:{}",
            idl_convert_to_type_name(idl_account_name)
        )))
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
}
