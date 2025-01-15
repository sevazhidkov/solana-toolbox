use solana_toolbox_endpoint::ToolboxEndpointError;

#[derive(Debug)]
pub enum ToolboxIdlError {
    ToolboxEndpoint(ToolboxEndpointError),
    Pubkey(solana_sdk::pubkey::PubkeyError),
    ParsePubkey(solana_sdk::pubkey::ParsePubkeyError),
    TryFromInt(std::num::TryFromIntError),
    TryFromSlice(std::array::TryFromSliceError),
    Inflate(String),
    FromUtf8(std::string::FromUtf8Error),
    SerdeJson(serde_json::Error),
    Overflow(),
    Custom(String),
}
