use std::collections::HashMap;

use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;
use crate::toolbox_idl_utils::idl_ok_or_else;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlAccount {
    pub name: String,
    pub value: Value,
}

impl ToolboxIdl {
    pub async fn get_accounts_by_name(
        &self,
        endpoint: &mut ToolboxEndpoint,
        accounts_addresses_by_name: &HashMap<String, Pubkey>,
    ) -> Result<HashMap<String, ToolboxIdlAccount>, ToolboxIdlError> {
        let mut accounts_names = vec![];
        let mut accounts_addresses = vec![];
        for (account_name, account_address) in accounts_addresses_by_name {
            accounts_names.push(account_name);
            accounts_addresses.push(*account_address);
        }
        let mut accounts_by_name = HashMap::new();
        for (account_name, account) in accounts_names
            .into_iter()
            .zip(endpoint.get_accounts(&accounts_addresses).await?)
        {
            if let Some(Ok(account)) =
                account.map(|account| self.decompile_account(&account.data))
            {
                accounts_by_name.insert(account_name.to_string(), account);
            }
        }
        Ok(accounts_by_name)
    }

    pub async fn get_accounts(
        &self,
        endpoint: &mut ToolboxEndpoint,
        accounts_addresses: &[Pubkey],
    ) -> Result<Vec<Option<ToolboxIdlAccount>>, ToolboxIdlError> {
        let mut accounts = vec![];
        for account in endpoint.get_accounts(accounts_addresses).await? {
            let account = account
                .map(|account| self.decompile_account(&account.data))
                .transpose()?;
            accounts.push(account);
        }
        Ok(accounts)
    }

    pub async fn get_account(
        &self,
        endpoint: &mut ToolboxEndpoint,
        account_address: &Pubkey,
    ) -> Result<Option<ToolboxIdlAccount>, ToolboxIdlError> {
        endpoint
            .get_account(account_address)
            .await?
            .map(|account| self.decompile_account(&account.data))
            .transpose()
    }

    pub fn compile_account(
        &self,
        account: &ToolboxIdlAccount,
    ) -> Result<Vec<u8>, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let program_account = idl_map_get_key_or_else(
            &self.program_accounts,
            &account.name,
            &breadcrumbs.as_idl("$program_accounts"),
        )?;
        let mut account_data = vec![];
        account_data.extend_from_slice(&program_account.discriminator);
        program_account.typedef.try_serialize(
            self,
            &account.value,
            &mut account_data,
            &breadcrumbs.with_idl(&account.name),
        )?;
        Ok(account_data)
    }

    pub fn decompile_account(
        &self,
        account_data: &[u8],
    ) -> Result<ToolboxIdlAccount, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let account_name = idl_ok_or_else(
            self.guess_account_name(account_data),
            "Could not guess account name",
            &breadcrumbs.as_val("account_name"),
        )?;
        let program_account = idl_map_get_key_or_else(
            &self.program_accounts,
            account_name,
            &breadcrumbs.as_idl("$program_accounts"),
        )?;
        if !account_data.starts_with(&program_account.discriminator) {
            return Err(ToolboxIdlError::InvalidDiscriminator {
                expected: program_account.discriminator.to_vec(),
                found: account_data.to_vec(),
            });
        }
        let (_, data_content_value) = program_account.typedef.try_deserialize(
            self,
            account_data,
            program_account.discriminator.len(),
            &breadcrumbs.with_idl(account_name),
        )?;
        Ok(ToolboxIdlAccount {
            name: account_name.to_string(),
            value: data_content_value,
        })
    }

    pub fn guess_account_name(
        &self,
        account_data: &[u8],
    ) -> Option<&str> {
        for (program_account_name, program_account) in &self.program_accounts {
            if account_data.starts_with(&program_account.discriminator) {
                return Some(program_account_name);
            }
        }
        None
    }
}
