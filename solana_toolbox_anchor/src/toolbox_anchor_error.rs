use solana_toolbox_endpoint::ToolboxEndpointError;

#[derive(Debug)]
pub enum ToolboxAnchorError {
    ToolboxEndpoint(ToolboxEndpointError),
    Pubkey(solana_sdk::pubkey::PubkeyError),
    ParsePubkey(solana_sdk::pubkey::ParsePubkeyError),
    Anchor(anchor_lang::error::Error),
    TryFromInt(std::num::TryFromIntError),
    TryFromSlice(std::array::TryFromSliceError),
    Inflate(String),
    FromUtf8(std::string::FromUtf8Error),
    SerdeJson(serde_json::Error),
    Overflow(),
    Custom(String),
}

impl From<ToolboxEndpointError> for ToolboxAnchorError {
    fn from(source: ToolboxEndpointError) -> Self {
        ToolboxAnchorError::ToolboxEndpoint(source)
    }
}
