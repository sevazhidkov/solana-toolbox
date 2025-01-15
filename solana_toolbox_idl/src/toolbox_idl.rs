use bytemuck::Pod;
use bytemuck::Zeroable;
use inflate::inflate_bytes_zlib;
use serde_json::from_str;
use serde_json::Map;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_anchor_endpoint::ToolboxAnchorEndpoint;
use crate::toolbox_anchor_error::ToolboxIdlError;
use crate::toolbox_anchor_idl_utils::idl_as_object_or_else;
use crate::toolbox_anchor_idl_utils::idl_object_get_key_as_array_or_else;
use crate::toolbox_anchor_idl_utils::idl_object_get_key_as_str;

#[derive(Debug, Clone)]
pub struct ToolboxIdl {
    pub program_id: Pubkey,
    pub authority: Pubkey,
    pub accounts: Map<String, Value>,
    pub types: Map<String, Value>,
    pub errors: Map<String, Value>,
    pub instructions_accounts: Map<String, Value>,
    pub instructions_args: Map<String, Value>,
}

impl ToolboxIdl {
    pub const DISCRIMINATOR: u64 = 42;

    pub async fn get_for_program_id(
        endpoint: &mut ToolboxEndpoint,
        program_id: &Pubkey,
    ) -> Result<Option<ToolboxIdl>, ToolboxIdlError> {
        let program_idl_address = ToolboxIdl::find_for_program_id(program_id)?;
        let program_idl_data =
            if let Some(account) = endpoint.get_account(&program_idl_address).await? {
                account.data
            }
            else {
                return Ok(None);
            };
        ToolboxAnchorIdl::try_from_bytes(program_idl_data)
    }

    pub fn find_for_program_id(
        program_id: &Pubkey,
    ) -> Result<Pubkey, ToolboxIdlError> {
        let base = Pubkey::find_program_address(&[], program_id).0;
        Pubkey::create_with_seed(&base, "anchor:idl", program_id)
            .map_err(ToolboxIdlError::Pubkey)
    }

    pub fn try_from_bytes(
        program_idl_data: &[u8],
    ) -> Result<ToolboxIdl, ToolboxIdlError> {
        #[derive(Debug, Clone, Copy, Pod, Zeroable)]
        #[repr(C)]
        struct ToolboxIdlHeader {
            discriminator: [u8; 8],
            authority: Pubkey,
            length: u32,
        }
        let data_content_offset = size_of::<ToolboxIdlHeader>();
        let data_header = bytemuck::from_bytes::<ToolboxIdlHeader>(
            &program_idl_data[0..data_content_offset],
        );
        if data_header.discriminator != ToolboxIdl::DISCRIMINATOR {
            return idl_err(
                "discriminator is invalid",
                format!("found:{:16X}", data_header.discriminator),
                format!("expected:{:16X}", ToolboxIdl::DISCRIMINATOR),
            );
        }
        let data_content_length = usize::try_from(data_header.length)
            .map_err(ToolboxIdlError::TryFromInt)?;
        let data_content_end = data_content_offset
            .checked_add(data_content_length)
            .ok_or_else(|| ToolboxIdlError::Overflow())?;
        let data_content_decompressed = inflate_bytes_zlib(
            &program_idl_data[data_content_offset..data_content_end],
        )
        .map_err(ToolboxIdlError::Inflate)?;
        let data_content_decoded = String::from_utf8(data_content_decompressed)
            .map_err(ToolboxIdlError::FromUtf8)?;
        let data_content_json = from_str::<Value>(&data_content_decoded)
            .map_err(ToolboxIdlError::SerdeJson)?;
        let data_content_object =
            idl_as_object_or_else(&data_content_json, "root")?;
        Ok(Some(ToolboxIdl {
            authority: data_header.authority,
            accounts: idl_collection_content_mapped_by_name(
                data_content_object,
                "accounts",
                "type",
            )?,
            types: idl_collection_content_mapped_by_name(
                data_content_object,
                "types",
                "type",
            )?,
            errors: idl_collection_content_mapped_by_name(
                data_content_object,
                "errors",
                "code",
            )?,
            instructions_accounts: idl_collection_content_mapped_by_name(
                data_content_object,
                "instructions",
                "accounts",
            )?,
            instructions_args: idl_collection_content_mapped_by_name(
                data_content_object,
                "instructions",
                "args",
            )?,
        }))
    }
}

fn idl_collection_content_mapped_by_name(
    object: &Map<String, Value>,
    collection_key: &str,
    content_key: &str,
) -> Result<Map<String, Value>, ToolboxIdlError> {
    let idl_array =
        idl_object_get_key_as_array_or_else(object, collection_key, "root")?;
    let mut object = Map::new();
    for idl_item in idl_array {
        if let Some(idl_item_object) = idl_item.as_object() {
            if let Some(item_name) =
                idl_object_get_key_as_str(&idl_item_object, "name")
            {
                if let Some(idl_item_content) = idl_item_object.get(content_key)
                {
                    object.insert(item_name.into(), idl_item_content.clone());
                }
            }
        }
    }
    Ok(object)
}
