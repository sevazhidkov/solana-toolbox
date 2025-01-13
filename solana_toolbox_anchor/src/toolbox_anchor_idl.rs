use bytemuck::Pod;
use bytemuck::Zeroable;
use inflate::inflate_bytes_zlib;
use serde_json::from_str;
use serde_json::Map;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_anchor_endpoint::ToolboxAnchorEndpoint;
use crate::toolbox_anchor_error::ToolboxAnchorError;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
struct ToolboxAnchorIdlHeader {
    discriminator: [u8; 8],
    authority: Pubkey,
    length: u32,
}

#[derive(Debug, Clone)]
pub struct ToolboxAnchorIdl {
    pub authority: Pubkey,
    pub json: Map<String, Value>,
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
            } else {
                return Ok(None);
            };
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
        let data_content_end = data_content_offset
            .checked_add(
                usize::try_from(data_header.length)
                    .map_err(ToolboxAnchorError::TryFromInt)?,
            )
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
            data_content_json.as_object().ok_or_else(|| {
                ToolboxAnchorError::Custom("IDL is not a json object".into())
            })?;
        Ok(Some(ToolboxAnchorIdl {
            authority: data_header.authority,
            json: data_content_object.to_owned(),
        }))
    }
}
