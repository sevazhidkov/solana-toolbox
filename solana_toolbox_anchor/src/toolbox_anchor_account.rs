use anchor_lang::AccountDeserialize;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointError;

pub async fn get_account_anchor_deserialized<T: AccountDeserialize>(
    endpoint: &mut ToolboxEndpoint,
    address: &Pubkey,
) -> Result<T, ToolboxAnchorError> {
    let raw_account_data = endpoint.get_account_data(address).await?;
    let mut raw_account_slice: &[u8] = &raw_account_data;
    T::try_deserialize(&mut raw_account_slice)
        .map_err(ToolboxAnchorError::Anchor)
}
