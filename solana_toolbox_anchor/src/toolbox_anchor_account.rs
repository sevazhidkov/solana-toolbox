use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_anchor::ToolboxAnchor;
use crate::toolbox_anchor_error::ToolboxAnchorError;

impl ToolboxAnchor {
    pub async fn get_account_data_deserialized<
        T: anchor_lang::AccountDeserialize,
    >(
        endpoint: &mut ToolboxEndpoint,
        address: &Pubkey,
    ) -> Result<Option<T>, ToolboxAnchorError> {
        endpoint
            .get_account(address)
            .await
            .map_err(ToolboxAnchorError::ToolboxEndpoint)?
            .map(|account| T::try_deserialize(&mut account.data.as_slice()))
            .transpose()
            .map_err(ToolboxAnchorError::Anchor)
    }
}
