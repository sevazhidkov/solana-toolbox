use bytemuck::Pod;
use bytemuck::Zeroable;
use inflate::inflate_bytes_zlib;
use serde_json::from_str;
use serde_json::Map;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_anchor_endpoint::ToolboxAnchorEndpoint;
use crate::toolbox_anchor_error::ToolboxAnchorError;
use crate::toolbox_anchor_idl_utils::idl_as_object_or_else;
use crate::toolbox_anchor_idl_utils::idl_object_get_key_as_array_or_else;
use crate::toolbox_anchor_idl_utils::idl_object_get_key_as_str;

#[derive(Debug, Clone)]
pub struct ToolboxAnchorIdl {
    pub program_id: Pubkey,
    pub authority: Pubkey,
    pub accounts: Map<String, Value>,
    pub types: Map<String, Value>,
    pub errors: Map<String, Value>,
    pub instructions_accounts: Map<String, Value>,
    pub instructions_args: Map<String, Value>,
}

impl ToolboxAnchorEndpoint {
    pub fn find_program_id_anchor_idl(
        &mut self,
        program_id: &Pubkey,
    ) -> Result<Pubkey, ToolboxAnchorError> {
        let base = Pubkey::find_program_address(&[], program_id).0;
        Pubkey::create_with_seed(&base, "anchor:idl", program_id)
            .map_err(ToolboxAnchorError::Pubkey)
    }

    pub async fn get_program_id_anchor_idl(
        &mut self,
        program_id: &Pubkey,
    ) -> Result<Option<ToolboxAnchorIdl>, ToolboxAnchorError> {
        let address = self.find_program_id_anchor_idl(program_id)?;
        let data_bytes =
            if let Some(account) = self.get_account(&address).await? {
                account.data
            }
            else {
                return Ok(None);
            };
        #[derive(Debug, Clone, Copy, Pod, Zeroable)]
        #[repr(C)]
        struct ToolboxAnchorIdlHeader {
            discriminator: [u8; 8],
            authority: Pubkey,
            length: u32,
        }
        let data_content_offset = size_of::<ToolboxAnchorIdlHeader>();
        let data_header = bytemuck::from_bytes::<ToolboxAnchorIdlHeader>(
            &data_bytes[0..data_content_offset],
        );
        if data_header.discriminator != [24, 70, 98, 191, 58, 144, 123, 158] {
            return Err(ToolboxAnchorError::Custom(format!(
                "Invalid IDL discriminator: {:?}",
                data_header.discriminator
            )));
        }
        let data_content_length = usize::try_from(data_header.length)
            .map_err(ToolboxAnchorError::TryFromInt)?;
        let data_content_end = data_content_offset
            .checked_add(data_content_length)
            .ok_or_else(|| ToolboxAnchorError::Overflow())?;
        let data_content_decompressed = inflate_bytes_zlib(
            &data_bytes[data_content_offset..data_content_end],
        )
        .map_err(ToolboxAnchorError::Inflate)?;
        let data_content_decoded = String::from_utf8(data_content_decompressed)
            .map_err(ToolboxAnchorError::FromUtf8)?;
        let data_content_json = from_str::<Value>(&data_content_decoded)
            .map_err(ToolboxAnchorError::SerdeJson)?;
        let data_content_object =
            idl_as_object_or_else(&data_content_json, "root")?;
        Ok(Some(ToolboxAnchorIdl {
            program_id: *program_id,
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
) -> Result<Map<String, Value>, ToolboxAnchorError> {
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
