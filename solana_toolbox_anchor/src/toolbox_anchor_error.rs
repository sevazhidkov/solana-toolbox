use solana_toolbox_endpoint::ToolboxEndpointError;

#[derive(Debug)]
pub enum ToolboxAnchorError {
    ToolboxEndpoint(ToolboxEndpointError),
    Anchor(anchor_lang::error::Error),
}

impl From<ToolboxEndpointError> for ToolboxAnchorError {
    fn from(source: ToolboxEndpointError) -> Self {
        ToolboxAnchorError::ToolboxEndpoint(source)
    }
}
