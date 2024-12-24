use borsh::BorshDeserialize;
use solana_sdk::account::Account;
use solana_sdk::program_pack::IsInitialized;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;

use crate::endpoint::Endpoint;
use crate::endpoint_error::EndpointError;

impl Endpoint {
    pub async fn get_account(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<Account>, EndpointError> {
        Ok(self.get_accounts(&[*address]).await?.pop().flatten())
    }

    pub async fn get_account_exists(
        &mut self,
        address: &Pubkey,
    ) -> Result<bool, EndpointError> {
        Ok(self.get_account(address).await.is_ok())
    }

    pub async fn get_account_lamports(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<u64>, EndpointError> {
        Ok(self
            .get_account(address)
            .await?
            .map(|account| account.lamports))
    }

    pub async fn get_account_owner(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<Pubkey>, EndpointError> {
        Ok(self
            .get_account(address)
            .await?
            .map(|account| account.owner))
    }

    pub async fn get_account_data(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<Vec<u8>>, EndpointError> {
        Ok(self.get_account(address).await?.map(|account| account.data))
    }

    pub async fn get_account_data_unpacked<T: Pack + IsInitialized>(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<T>, EndpointError> {
        self.get_account(address)
            .await?
            .map(|account| T::unpack(&account.data))
            .transpose()
            .map_err(EndpointError::Program)
    }

    pub async fn get_account_data_borsh_deserialized<T: BorshDeserialize>(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<T>, EndpointError> {
        self.get_account(address)
            .await?
            .map(|account| T::try_from_slice(&account.data))
            .transpose()
            .map_err(EndpointError::Io)
    }
}
