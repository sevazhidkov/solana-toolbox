#[derive(Debug)]
pub enum ToolboxAnchorError {
    ToolboxEndpoint(solana_toolbox_endpoint::ToolboxEndpointError),
    Pubkey(solana_sdk::pubkey::PubkeyError),
    Anchor(anchor_lang::error::Error),
}
