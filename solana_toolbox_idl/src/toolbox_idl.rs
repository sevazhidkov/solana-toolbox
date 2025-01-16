use bytemuck::AnyBitPattern;
use inflate::inflate_bytes_zlib;
use serde_json::from_str;
use serde_json::Map;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_object_get_key_as_array_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;

#[derive(Debug, Clone)]
pub struct ToolboxIdl {
    pub accounts: Map<String, Value>,
    pub types: Map<String, Value>,
    pub errors: Map<String, Value>,
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
        let disciminator = read_from_bytes_at::<u64>(&data, 0)?;
        if *disciminator != ToolboxIdl::DISCRIMINATOR {
            return idl_err(&format!(
                "discriminator is invalid: found {:016X}, expected {:016X}",
                disciminator,
                ToolboxIdl::DISCRIMINATOR
            ));
        }
        let authority_offset = size_of_val(disciminator);
        let authority = read_from_bytes_at::<Pubkey>(&data, authority_offset)?;
        let length_offset = authority_offset + size_of_val(authority);
        let length =
            usize::try_from(*read_from_bytes_at::<u32>(&data, length_offset)?)
                .map_err(ToolboxIdlError::TryFromInt)?;
        let content_offset = length_offset + size_of_val(&length);
        let content = &data[content_offset
            ..content_offset
                .checked_add(length)
                .ok_or_else(ToolboxIdlError::Overflow)?];
        let decompressed =
            inflate_bytes_zlib(content).map_err(ToolboxIdlError::Inflate)?;
        let decoded = String::from_utf8(decompressed)
            .map_err(ToolboxIdlError::FromUtf8)?;
        ToolboxIdl::try_from_str(&decoded)
    }

    pub fn try_from_str(content: &str) -> Result<ToolboxIdl, ToolboxIdlError> {
        let idl_root_value =
            from_str::<Value>(&content).map_err(ToolboxIdlError::SerdeJson)?;
        let idl_root_object = idl_as_object_or_else(&idl_root_value, "root")?;
        Ok(ToolboxIdl {
            accounts: idl_collection_content_mapped_by_name(
                idl_root_object,
                "accounts",
                "type",
            )?,
            types: idl_collection_content_mapped_by_name(
                idl_root_object,
                "types",
                "type",
            )?,
            errors: idl_collection_content_mapped_by_name(
                idl_root_object,
                "errors",
                "code",
            )?,
            instructions_accounts: idl_collection_content_mapped_by_name(
                idl_root_object,
                "instructions",
                "accounts",
            )?,
            instructions_args: idl_collection_content_mapped_by_name(
                idl_root_object,
                "instructions",
                "args",
            )?,
        })
    }
}

fn read_from_bytes_at<'a, T: AnyBitPattern>(
    bytes: &'a [u8],
    offset: usize,
) -> Result<&'a T, ToolboxIdlError> {
    Ok(bytemuck::from_bytes::<T>(
        &bytes[offset
            ..offset
                .checked_add(size_of::<T>())
                .ok_or_else(ToolboxIdlError::Overflow)?],
    ))
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
                idl_object_get_key_as_str(idl_item_object, "name")
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
