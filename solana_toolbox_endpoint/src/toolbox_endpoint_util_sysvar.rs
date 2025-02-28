use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::slot_hashes::SlotHashes;
use solana_sdk::sysvar::clock;
use solana_sdk::sysvar::clock::Clock;
use solana_sdk::sysvar::rent;
use solana_sdk::sysvar::rent::Rent;
use solana_sdk::sysvar::slot_hashes;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_error::ToolboxEndpointError;

impl ToolboxEndpoint {
    pub const SYSVAR_CLOCK_ID: Pubkey = clock::ID;
    pub const SYSVAR_RENT_ID: Pubkey = rent::ID;
    pub const SYSVAR_SLOT_HASHES: Pubkey = slot_hashes::ID;

    pub async fn get_sysvar_clock(
        &mut self
    ) -> Result<Clock, ToolboxEndpointError> {
        self.get_account_data_bincode_deserialized(
            &ToolboxEndpoint::SYSVAR_CLOCK_ID,
        )
        .await?
        .ok_or_else(|| {
            ToolboxEndpointError::Custom("sysvar clock not found".to_string())
        })
    }

    pub async fn get_sysvar_rent(
        &mut self
    ) -> Result<Rent, ToolboxEndpointError> {
        self.get_account_data_bincode_deserialized(
            &ToolboxEndpoint::SYSVAR_RENT_ID,
        )
        .await?
        .ok_or_else(|| {
            ToolboxEndpointError::Custom("sysvar rent not found".to_string())
        })
    }

    pub async fn get_sysvar_slot_hashes(
        &mut self
    ) -> Result<Vec<(u64, Hash)>, ToolboxEndpointError> {
        let slot_hashes: SlotHashes = self
            .get_account_data_bincode_deserialized(
                &ToolboxEndpoint::SYSVAR_SLOT_HASHES,
            )
            .await?
            .ok_or_else(|| {
                ToolboxEndpointError::Custom(
                    "sysvar slot_hashes not found".to_string(),
                )
            })?;
        Ok(slot_hashes.slot_hashes().to_vec())
    }
}
