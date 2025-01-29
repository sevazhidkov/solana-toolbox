use solana_sdk::account::Account;
use solana_sdk::program_pack::IsInitialized;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_error::ToolboxEndpointError;

impl ToolboxEndpoint {
    pub async fn get_account_exists(
        &mut self,
        address: &Pubkey,
    ) -> Result<bool, ToolboxEndpointError> {
        Ok(self.get_account_lamports(address).await?.is_some())
    }

    pub async fn get_account_lamports(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<u64>, ToolboxEndpointError> {
        Ok(match self.get_balance(address).await? {
            0 => None,
            balance => Some(balance),
        })
    }

    pub async fn get_account_or_default(
        &mut self,
        address: &Pubkey,
    ) -> Result<Account, ToolboxEndpointError> {
        Ok(self.get_account(address).await?.unwrap_or_default())
    }

    pub async fn get_account(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<Account>, ToolboxEndpointError> {
        Ok(self.get_accounts(&[*address]).await?.pop().flatten())
    }

    pub async fn get_account_owner(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<Pubkey>, ToolboxEndpointError> {
        Ok(self.get_account(address).await?.map(|account| account.owner))
    }

    pub async fn get_account_data(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<Vec<u8>>, ToolboxEndpointError> {
        Ok(self.get_account(address).await?.map(|account| account.data))
    }

    pub async fn get_account_executable(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<bool>, ToolboxEndpointError> {
        Ok(self.get_account(address).await?.map(|account| account.executable))
    }

    pub async fn get_account_data_unpacked<T: Pack + IsInitialized>(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<T>, ToolboxEndpointError> {
        Ok(self
            .get_account(address)
            .await?
            .map(|account| T::unpack(&account.data))
            .transpose()?)
    }

    pub async fn get_account_data_bincode_deserialized<
        T: for<'a> serde::Deserialize<'a>,
    >(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<T>, ToolboxEndpointError> {
        self.get_account(address)
            .await?
            .map(|account| bincode::deserialize::<T>(&account.data))
            .transpose()
            .map_err(ToolboxEndpointError::Bincode)
    }

    pub async fn get_account_data_borsh_deserialized<
        T: borsh::BorshDeserialize,
    >(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<T>, ToolboxEndpointError> {
        self.get_account(address)
            .await?
            .map(|account| T::try_from_slice(&account.data))
            .transpose()
            .map_err(ToolboxEndpointError::Io)
    }

    pub async fn get_account_data_bytemuck_casted<
        T: bytemuck::AnyBitPattern,
    >(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<T>, ToolboxEndpointError> {
        self.get_account(address)
            .await?
            .map(|account| {
                bytemuck::try_from_bytes::<T>(&account.data).cloned()
            })
            .transpose()
            .map_err(ToolboxEndpointError::PodCastError)
    }

    pub async fn get_account_data_bytemuck_casted_at<
        T: bytemuck::AnyBitPattern,
    >(
        &mut self,
        address: &Pubkey,
        offset: usize,
    ) -> Result<Option<T>, ToolboxEndpointError> {
        self.get_account(address)
            .await?
            .map(|account| {
                let data_size = offset + std::mem::size_of::<T>();
                if account.data.len() < data_size {
                    return Err(ToolboxEndpointError::Custom(
                        "Account is too small".into(),
                    ));
                }
                bytemuck::try_from_bytes::<T>(&account.data[offset..data_size])
                    .cloned()
                    .map_err(ToolboxEndpointError::PodCastError)
            })
            .transpose()
    }
}
