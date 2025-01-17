use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;
use crate::toolbox_idl_utils::idl_u64_from_bytes_at;

impl ToolboxIdl {
    pub async fn get_account(
        &self,
        endpoint: &mut ToolboxEndpoint,
        account_type: &str,
        account_address: &Pubkey,
    ) -> Result<Option<(usize, Value)>, ToolboxIdlError> {
        let account_data = match endpoint.get_account(account_address).await? {
            Some(account) => account.data,
            None => return Ok(None),
        };
        Ok(Some(self.decompile_account(account_type, &account_data)?))
    }

    pub fn decompile_account(
        &self,
        account_type: &str,
        account_data: &[u8],
    ) -> Result<(usize, Value), ToolboxIdlError> {
        let idl_type = match self.account_types.get(account_type) {
            Some(idl_account_type) => idl_account_type,
            None => {
                idl_object_get_key_or_else(&self.types, account_type, "types")?
            },
        };
        let data_discriminator = idl_u64_from_bytes_at(account_data, 0)?;
        let expected_discriminator =
            ToolboxIdl::compute_account_discriminator(account_type);
        if data_discriminator != expected_discriminator {
            return idl_err(&format!(
                "invalid discriminator: found {:016X}, expected {:016X}",
                data_discriminator, expected_discriminator
            ));
        }
        let data_header_size = size_of_val(&data_discriminator);
        let (data_content_size, data_content_value) =
            self.type_reader(idl_type, account_data, data_header_size)?;
        Ok((data_header_size + data_content_size, data_content_value))
    }

    pub fn compile_account(
        &self,
        account_type: &str,
        account_value: &Value,
    ) -> Result<Vec<u8>, ToolboxIdlError> {
        let mut account_data = vec![];
        account_data.extend_from_slice(bytemuck::bytes_of(
            &ToolboxIdl::compute_account_discriminator(account_type),
        ));
        let idl_type = match self.account_types.get(account_type) {
            Some(idl_account_type) => idl_account_type,
            None => {
                idl_object_get_key_or_else(&self.types, account_type, "types")?
            },
        };
        self.type_writer(idl_type, account_value, &mut account_data)?;
        Ok(account_data)
    }
}
