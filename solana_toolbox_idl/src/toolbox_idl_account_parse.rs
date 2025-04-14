use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_account::ToolboxIdlAccount;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_typedef::ToolboxIdlTypedef;
use crate::toolbox_idl_utils::idl_as_bytes_or_else;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_hash_discriminator_from_string;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_array_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_u64;
use crate::toolbox_idl_utils::idl_object_get_key_as_u64_or_else;

impl ToolboxIdlAccount {
    pub fn try_parse(
        idl_account_name: &str,
        idl_account: &Value,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlAccount> {
        let idl_account = idl_as_object_or_else(idl_account)?;
        let discriminator = ToolboxIdlAccount::try_parse_discriminator(
            idl_account_name,
            idl_account,
        )?;
        let docs = idl_account.get("docs").cloned();
        let space = idl_object_get_key_as_u64(idl_account, "space")
            .map(usize::try_from)
            .transpose()?;
        let blobs = ToolboxIdlAccount::try_parse_blobs(idl_account)?;
        let content_type_flat = ToolboxIdlAccount::try_parse_data_type_flat(
            idl_account_name,
            idl_account,
        )?;
        let content_type_full =
            content_type_flat.try_hydrate(&HashMap::new(), typedefs)?;
        Ok(ToolboxIdlAccount {
            name: idl_account_name.to_string(),
            docs,
            space,
            blobs,
            discriminator,
            content_type_flat,
            content_type_full,
        })
    }

    fn try_parse_discriminator(
        idl_account_name: &str,
        idl_account: &Map<String, Value>,
    ) -> Result<Vec<u8>> {
        if let Some(idl_account_discriminator) =
            idl_object_get_key_as_array(idl_account, "discriminator")
        {
            return idl_as_bytes_or_else(idl_account_discriminator);
        }
        Ok(idl_hash_discriminator_from_string(&format!(
            "account:{}",
            idl_account_name
        )))
    }

    fn try_parse_blobs(
        idl_account: &Map<String, Value>,
    ) -> Result<Vec<(usize, Vec<u8>)>> {
        let mut blobs = vec![];
        if let Some(idl_account_blobs) =
            idl_object_get_key_as_array(idl_account, "blobs")
        {
            for idl_account_blob in idl_account_blobs {
                if let Some(idl_account_blob) = idl_account_blob.as_object() {
                    let offset =
                        usize::try_from(idl_object_get_key_as_u64_or_else(
                            idl_account_blob,
                            "offset",
                        )?)?;
                    let bytes = idl_as_bytes_or_else(
                        idl_object_get_key_as_array_or_else(
                            idl_account_blob,
                            "value",
                        )?,
                    )?;
                    blobs.push((offset, bytes));
                }
            }
        }
        Ok(blobs)
    }

    fn try_parse_data_type_flat(
        idl_account_name: &str,
        idl_account: &Map<String, Value>,
    ) -> Result<ToolboxIdlTypeFlat> {
        if idl_account.contains_key("type")
            || idl_account.contains_key("defined")
            || idl_account.contains_key("generic")
            || idl_account.contains_key("option")
            || idl_account.contains_key("option32")
            || idl_account.contains_key("vec") // TODO (FAR) - should we support vec8/variants32 ??
            || idl_account.contains_key("array")
            || idl_account.contains_key("fields")
            || idl_account.contains_key("variants")
            || idl_account.contains_key("padded")
        {
            return ToolboxIdlTypeFlat::try_parse_object(idl_account);
        }
        Ok(ToolboxIdlTypeFlat::Defined {
            name: idl_account_name.to_string(),
            generics: vec![],
        })
    }
}
