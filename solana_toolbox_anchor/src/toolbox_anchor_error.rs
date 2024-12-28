#[derive(Debug)]
pub enum ToolboxAnchorError {
    ToolboxEndpoint(solana_toolbox_endpoint::ToolboxEndpointError),
    Anchor(anchor_lang::error::Error),
}
