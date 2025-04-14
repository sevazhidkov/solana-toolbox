use anyhow::anyhow;
use anyhow::Result;
use solana_sdk::account::Account;
use solana_sdk::program_pack::IsInitialized;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_endpoint::ToolboxEndpoint;

impl ToolboxEndpoint {
    pub async fn get_account_or_default(
        &mut self,
        address: &Pubkey,
    ) -> Result<Account> {
        Ok(self.get_account(address).await?.unwrap_or_default())
    }

    pub async fn get_account_exists(
        &mut self,
        address: &Pubkey,
    ) -> Result<bool> {
        Ok(self.get_account_lamports(address).await?.is_some())
    }

    pub async fn get_account_lamports(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<u64>> {
        Ok(match self.get_balance(address).await? {
            0 => None,
            balance => Some(balance),
        })
    }

    pub async fn get_account_owner(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<Pubkey>> {
        Ok(self
            .get_account(address)
            .await?
            .map(|account| account.owner))
    }

    pub async fn get_account_data(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<Vec<u8>>> {
        Ok(self.get_account(address).await?.map(|account| account.data))
    }

    pub async fn get_account_executable(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<bool>> {
        Ok(self
            .get_account(address)
            .await?
            .map(|account| account.executable))
    }

    pub async fn get_account_data_unpacked<T: Pack + IsInitialized>(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<T>> {
        self.get_account_data_unpacked_at(address, 0).await
    }

    pub async fn get_account_data_unpacked_at<T: Pack + IsInitialized>(
        &mut self,
        address: &Pubkey,
        offset: usize,
    ) -> Result<Option<T>> {
        Ok(self
            .get_account(address)
            .await?
            .map(|account| T::unpack(&account.data[offset..]))
            .transpose()?)
    }

    pub async fn get_account_data_bincode_deserialized<
        T: for<'a> serde::Deserialize<'a>,
    >(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<T>> {
        self.get_account_data_bincode_deserialized_slice(address, 0, None)
            .await
    }

    pub async fn get_account_data_bincode_deserialized_slice<
        T: for<'a> serde::Deserialize<'a>,
    >(
        &mut self,
        address: &Pubkey,
        offset: usize,
        length: Option<usize>,
    ) -> Result<Option<T>> {
        Ok(self
            .get_account(address)
            .await?
            .map(|account| {
                bincode::deserialize::<T>(
                    &account.data[offset
                        ..offset
                            + length.unwrap_or(account.data.len() - offset)],
                )
            })
            .transpose()?)
    }

    pub async fn get_account_data_borsh_deserialized<
        T: borsh::BorshDeserialize,
    >(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<T>> {
        self.get_account_data_borsh_deserialized_slice(address, 0, None)
            .await
    }

    pub async fn get_account_data_borsh_deserialized_slice<
        T: borsh::BorshDeserialize,
    >(
        &mut self,
        address: &Pubkey,
        offset: usize,
        length: Option<usize>,
    ) -> Result<Option<T>> {
        Ok(self
            .get_account(address)
            .await?
            .map(|account| {
                T::try_from_slice(
                    &account.data[offset
                        ..offset
                            + length.unwrap_or(account.data.len() - offset)],
                )
            })
            .transpose()?)
    }

    pub async fn get_account_data_bytemuck_casted<
        T: bytemuck::AnyBitPattern,
    >(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<T>> {
        self.get_account_data_bytemuck_casted_at(address, 0).await
    }

    pub async fn get_account_data_bytemuck_casted_at<
        T: bytemuck::AnyBitPattern,
    >(
        &mut self,
        address: &Pubkey,
        offset: usize,
    ) -> Result<Option<T>> {
        self.get_account(address)
            .await?
            .map(|account| {
                bytemuck::try_from_bytes::<T>(&account.data[offset..]).cloned()
            })
            .transpose()
            .map_err(|error| {
                anyhow!("Could not bytemuck deserialize data: {}", error)
            })
    }
}
