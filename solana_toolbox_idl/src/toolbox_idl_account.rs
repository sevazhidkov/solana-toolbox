use std::collections::HashMap;

use serde_json::Map;
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
    pub async fn get_accounts_values_by_name(
        &self,
        endpoint: &mut ToolboxEndpoint,
        accounts_addresses_by_name: &HashMap<String, Pubkey>,
    ) -> Result<Map<String, Value>, ToolboxIdlError> {
        let mut accounts_names = vec![];
        let mut accounts_addresses = vec![];
        for (account_name, account_address) in accounts_addresses_by_name {
            accounts_names.push(account_name);
            accounts_addresses.push(*account_address);
        }
        let mut accounts_values_by_name = Map::new();
        for (account_name, account_info) in accounts_names
            .into_iter()
            .zip(endpoint.get_accounts(&accounts_addresses).await?)
        {
            if let Some(Ok(account_value)) = account_info
                .map(|account| self.parse_account_value(&account.data))
            {
                accounts_values_by_name
                    .insert(account_name.to_string(), account_value);
            }
        }
        Ok(accounts_values_by_name)
    }

    pub async fn get_accounts_values(
        &self,
        endpoint: &mut ToolboxEndpoint,
        account_addresses: &[Pubkey],
    ) -> Result<Vec<Option<Value>>, ToolboxIdlError> {
        let mut accounts_values = vec![];
        for account in endpoint.get_accounts(account_addresses).await? {
            let account_value = account
                .map(|account| self.parse_account_value(&account.data))
                .transpose()?;
            accounts_values.push(account_value);
        }
        Ok(accounts_values)
    }

    pub async fn get_account_value(
        &self,
        endpoint: &mut ToolboxEndpoint,
        account_address: &Pubkey,
    ) -> Result<Option<Value>, ToolboxIdlError> {
        endpoint
            .get_account(account_address)
            .await?
            .map(|account| self.parse_account_value(&account.data))
            .transpose()
    }

    pub fn parse_account_value(
        &self,
        account_data: &[u8],
    ) -> Result<Value, ToolboxIdlError> {
        let account_name = idl_ok_or_else(
            self.guess_account_name(account_data),
            "Could not guess account name",
            &ToolboxIdlBreadcrumbs::default().as_val("account_name"),
        )?;
        Ok(self.decompile_account(account_name, account_data)?.1)
    }

    pub fn decompile_account(
        &self,
        account_name: &str,
        account_data: &[u8],
    ) -> Result<(usize, Value), ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let discriminator = idl_map_get_key_or_else(
            &self.accounts_discriminators,
            account_name,
            &breadcrumbs.as_idl("accounts_discriminators"),
        )?;
        if !account_data.starts_with(discriminator) {
            return Err(ToolboxIdlError::InvalidDiscriminator {
                expected: discriminator.to_vec(),
                found: account_data.to_vec(),
            });
        }
        let idl_account_type = idl_object_get_key_or_else(
            &self.accounts_types,
            account_name,
            &breadcrumbs.as_idl("accounts_types"),
        )?;
        let data_header_size = discriminator.len();
        let (data_content_size, data_content_value) = self.type_deserialize(
            idl_account_type,
            account_data,
            data_header_size,
            &breadcrumbs.with_idl(account_name),
        )?;
        Ok((data_header_size + data_content_size, data_content_value))
    }

    pub fn compile_account(
        &self,
        account_name: &str,
        account_value: &Value,
    ) -> Result<Vec<u8>, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let discriminator = idl_map_get_key_or_else(
            &self.accounts_discriminators,
            account_name,
            &breadcrumbs.as_idl("accounts_discriminators"),
        )?;
        let mut account_data = vec![];
        account_data.extend_from_slice(discriminator);
        let idl_account_type = idl_object_get_key_or_else(
            &self.accounts_types,
            account_name,
            &breadcrumbs.as_idl("accounts_types"),
        )?;
        self.type_serialize(
            idl_account_type,
            account_value,
            &mut account_data,
            &breadcrumbs.with_idl(account_name),
        )?;
        Ok(account_data)
    }

    pub fn guess_account_name(
        &self,
        account_data: &[u8],
    ) -> Option<&str> {
        for (account_name, account_discriminator) in
            &self.accounts_discriminators
        {
            if account_data.starts_with(account_discriminator) {
                return Some(account_name);
            }
        }
        None
    }
}
