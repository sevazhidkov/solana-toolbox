use anyhow::Result;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_anchor::ToolboxAnchor;

impl ToolboxAnchor {
    pub async fn get_account_data_deserialized<
        T: anchor_lang::AccountDeserialize,
    >(
        endpoint: &mut ToolboxEndpoint,
        address: &Pubkey,
    ) -> Result<Option<T>> {
        Ok(endpoint
            .get_account(address)
            .await?
            .map(|account| T::try_deserialize(&mut account.data.as_slice()))
            .transpose()?)
    }
}
