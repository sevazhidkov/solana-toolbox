use solana_toolbox_endpoint::ToolboxEndpointError;

#[derive(Debug)]
pub enum ToolboxAnchorError {
    ToolboxEndpoint(ToolboxEndpointError),
    Anchor(anchor_lang::error::Error),
    Custom(String),
}

impl From<ToolboxEndpointError> for ToolboxAnchorError {
    fn from(source: ToolboxEndpointError) -> Self {
        ToolboxAnchorError::ToolboxEndpoint(source)
    }
}

impl From<anchor_lang::error::Error> for ToolboxAnchorError {
    fn from(source: anchor_lang::error::Error) -> Self {
        ToolboxAnchorError::Anchor(source)
    }
}
