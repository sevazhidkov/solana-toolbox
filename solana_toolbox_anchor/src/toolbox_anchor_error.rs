use solana_toolbox_endpoint::ToolboxEndpointError;

#[derive(Debug)]
pub enum ToolboxAnchorError {
    ToolboxEndpoint(ToolboxEndpointError),
    Pubkey(solana_sdk::pubkey::PubkeyError),
    Anchor(anchor_lang::error::Error),
}

impl From<ToolboxEndpointError> for ToolboxAnchorError {
    fn from(content: ToolboxEndpointError) -> Self {
        ToolboxAnchorError::ToolboxEndpoint(content)
    }
}
