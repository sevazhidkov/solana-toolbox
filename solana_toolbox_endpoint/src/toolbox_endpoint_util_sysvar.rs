use solana_sdk::sysvar::clock;
use solana_sdk::sysvar::clock::Clock;
use solana_sdk::sysvar::rent;
use solana_sdk::sysvar::rent::Rent;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_error::ToolboxEndpointError;

impl ToolboxEndpoint {
    pub async fn get_sysvar_clock(
        &mut self
    ) -> Result<Clock, ToolboxEndpointError> {
        self.get_account_data_bincode_deserialized(&clock::ID)
            .await?
            .ok_or_else(|| {
                ToolboxEndpointError::Custom("sysvar clock not found")
            })
    }

    pub async fn get_sysvar_rent(
        &mut self
    ) -> Result<Rent, ToolboxEndpointError> {
        self.get_account_data_bincode_deserialized(&rent::ID).await?.ok_or_else(
            || ToolboxEndpointError::Custom("sysvar rent not found"),
        )
    }
}
