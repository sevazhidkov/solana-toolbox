use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;
use crate::toolbox_idl_utils::idl_ok_or_else;

impl ToolboxIdl {
    pub async fn get_account_value(
        &self,
        endpoint: &mut ToolboxEndpoint,
        account_address: &Pubkey,
    ) -> Result<Option<Value>, ToolboxIdlError> {
        let account_data = match endpoint.get_account(account_address).await? {
            Some(account) => account.data,
            None => return Ok(None),
        };
        let account_type = idl_ok_or_else(
            self.guess_account_type(&account_data),
            "Unknown account type",
            &ToolboxIdlBreadcrumbs::default().as_val("account_type"),
        )?;
        Ok(Some(self.decompile_account(account_type, &account_data)?.1))
    }

    pub fn guess_account_type(
        &self,
        account_data: &[u8],
    ) -> Option<&str> {
        for (account_type, account_discriminator) in
            &self.accounts_discriminators
        {
            if account_data.starts_with(account_discriminator) {
                return Some(account_type);
            }
        }
        None
    }

    pub fn decompile_account(
        &self,
        account_type: &str,
        account_data: &[u8],
    ) -> Result<(usize, Value), ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let discriminator = idl_map_get_key_or_else(
            &self.accounts_discriminators,
            account_type,
            &breadcrumbs.as_idl("accounts_discriminators"),
        )?;
        if !account_data.starts_with(discriminator) {
            return Err(ToolboxIdlError::InvalidDiscriminator {
                expected: discriminator.to_vec(),
                found: account_data.to_vec(),
            });
        }
        let idl_type = match self.accounts_types.get(account_type) {
            Some(idl_account_type) => idl_account_type,
            None => {
                idl_object_get_key_or_else(
                    &self.types,
                    account_type,
                    &breadcrumbs.as_idl("types"),
                )?
            },
        };
        let data_header_size = discriminator.len();
        let (data_content_size, data_content_value) = self.type_deserialize(
            idl_type,
            account_data,
            data_header_size,
            &breadcrumbs.with_idl(account_type),
        )?;
        Ok((data_header_size + data_content_size, data_content_value))
    }

    pub fn compile_account(
        &self,
        account_type: &str,
        account_value: &Value,
    ) -> Result<Vec<u8>, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let discriminator = idl_map_get_key_or_else(
            &self.accounts_discriminators,
            account_type,
            &breadcrumbs.as_idl("accounts_discriminators"),
        )?;
        let mut account_data = vec![];
        account_data.extend_from_slice(discriminator);
        let idl_type = match self.accounts_types.get(account_type) {
            Some(idl_account_type) => idl_account_type,
            None => {
                idl_object_get_key_or_else(
                    &self.types,
                    account_type,
                    &breadcrumbs.as_idl("types"),
                )?
            },
        };
        self.type_serialize(
            idl_type,
            account_value,
            &mut account_data,
            &breadcrumbs.with_idl(account_type),
        )?;
        Ok(account_data)
    }
}
