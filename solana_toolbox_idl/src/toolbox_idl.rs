use inflate::inflate_bytes_zlib;
use serde_json::from_str;
use serde_json::Map;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_array_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_pubkey_from_bytes_at;
use crate::toolbox_idl_utils::idl_slice_from_bytes;
use crate::toolbox_idl_utils::idl_u32_from_bytes_at;
use crate::toolbox_idl_utils::idl_u64_from_bytes_at;

#[derive(Debug, Clone)]
pub struct ToolboxIdl {
    pub types: Map<String, Value>,
    pub account_types: Map<String, Value>,
    pub errors_codes: Map<String, Value>,
    pub instructions_accounts: Map<String, Value>,
    pub instructions_args: Map<String, Value>,
}

impl ToolboxIdl {
    pub async fn get_for_program_id(
        endpoint: &mut ToolboxEndpoint,
        program_id: &Pubkey,
    ) -> Result<Option<ToolboxIdl>, ToolboxIdlError> {
        let address = &ToolboxIdl::find_for_program_id(program_id)?;
        let data = match endpoint.get_account(address).await? {
            Some(account) => account.data,
            None => return Ok(None),
        };
        Ok(Some(ToolboxIdl::try_from_bytes(&data)?))
    }

    pub fn find_for_program_id(
        program_id: &Pubkey
    ) -> Result<Pubkey, ToolboxIdlError> {
        let base = Pubkey::find_program_address(&[], program_id).0;
        Pubkey::create_with_seed(&base, "anchor:idl", program_id)
            .map_err(ToolboxIdlError::Pubkey)
    }

    pub fn try_from_bytes(data: &[u8]) -> Result<ToolboxIdl, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let discriminator_offset = 0;
        let disciminator = idl_u64_from_bytes_at(
            data,
            discriminator_offset,
            &breadcrumbs.as_val("discriminator"),
        )?;
        if disciminator != ToolboxIdl::DISCRIMINATOR {
            return Err(ToolboxIdlError::InvalidDiscriminator {
                found: disciminator,
                expected: ToolboxIdl::DISCRIMINATOR,
            });
        }
        let authority_offset = size_of_val(&disciminator);
        let authority = idl_pubkey_from_bytes_at(
            data,
            authority_offset,
            &breadcrumbs.as_val("authority"),
        )?;
        let length_offset = authority_offset + size_of_val(&authority);
        let length = idl_u32_from_bytes_at(
            data,
            length_offset,
            &breadcrumbs.as_val("length"),
        )?;
        let content_offset = length_offset + size_of_val(&length);
        let content = idl_slice_from_bytes(
            data,
            content_offset,
            usize::try_from(length).map_err(|err| {
                ToolboxIdlError::InvalidInteger {
                    conversion: err,
                    context: breadcrumbs.as_val("length"),
                }
            })?,
            &breadcrumbs.as_val("content"),
        )?;
        let content_encoded =
            inflate_bytes_zlib(content).map_err(ToolboxIdlError::Inflate)?;
        let content_decoded =
            String::from_utf8(content_encoded).map_err(|err| {
                ToolboxIdlError::InvalidString {
                    parsing: err,
                    context: breadcrumbs.as_val("content"),
                }
            })?;
        ToolboxIdl::try_from_str(&content_decoded)
    }

    pub fn try_from_str(content: &str) -> Result<ToolboxIdl, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let idl_root_value =
            from_str::<Value>(content).map_err(ToolboxIdlError::SerdeJson)?;
        let idl_root_object =
            idl_as_object_or_else(&idl_root_value, &breadcrumbs.as_idl("$"))?;
        Ok(ToolboxIdl {
            types: idl_collection_content_mapped_by_name(
                idl_root_object,
                "types",
                "type",
                breadcrumbs,
            )?,
            account_types: idl_collection_content_mapped_by_name(
                idl_root_object,
                "accounts",
                "type",
                breadcrumbs,
            )?,
            errors_codes: idl_collection_content_mapped_by_name(
                idl_root_object,
                "errors",
                "code",
                breadcrumbs,
            )?,
            instructions_accounts: idl_collection_content_mapped_by_name(
                idl_root_object,
                "instructions",
                "accounts",
                breadcrumbs,
            )?,
            instructions_args: idl_collection_content_mapped_by_name(
                idl_root_object,
                "instructions",
                "args",
                breadcrumbs,
            )?,
        })
    }
}

fn idl_collection_content_mapped_by_name(
    object: &Map<String, Value>,
    collection_key: &str,
    content_key: &str,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<Map<String, Value>, ToolboxIdlError> {
    let idl_array = idl_object_get_key_as_array_or_else(
        object,
        collection_key,
        &breadcrumbs.as_idl("root"),
    )?;
    let mut idl_collection = Map::new();
    for idl_item in idl_array {
        if let Some(idl_item_object) = idl_item.as_object() {
            if let Some(item_name) =
                idl_object_get_key_as_str(idl_item_object, "name")
            {
                if let Some(idl_item_content) = idl_item_object.get(content_key)
                {
                    idl_collection
                        .insert(item_name.into(), idl_item_content.clone());
                }
            }
        }
    }
    Ok(idl_collection)
}
