use solana_sdk::pubkey::Pubkey;

use crate::toolbox_anchor_endpoint::ToolboxAnchorEndpoint;
use crate::toolbox_anchor_error::ToolboxAnchorError;

impl ToolboxAnchorEndpoint {
    pub async fn get_account_data_anchor_deserialized<
        T: anchor_lang::AccountDeserialize,
    >(
        &mut self,
        address: &Pubkey,
    ) -> Result<T, ToolboxAnchorError> {
        let account_data = self
            .get_account_data(address)
            .await
            .map_err(ToolboxAnchorError::ToolboxEndpoint)?;
        T::try_deserialize(&mut account_data.as_slice())
            .map_err(ToolboxAnchorError::Anchor)
    }
}
