use solana_sdk::pubkey::ParsePubkeyError;
use solana_toolbox_endpoint::ToolboxEndpointError;
use solana_toolbox_idl::ToolboxIdlError;

#[derive(Debug)]
pub enum ToolboxCliError {
    ToolboxEndpoint(ToolboxEndpointError),
    ToolboxIdl(ToolboxIdlError),
    ParsePubkey(ParsePubkeyError),
    SerdeJson(serde_json::Error),
}

impl From<ToolboxEndpointError> for ToolboxCliError {
    fn from(source: ToolboxEndpointError) -> Self {
        ToolboxCliError::ToolboxEndpoint(source)
    }
}

impl From<ToolboxIdlError> for ToolboxCliError {
    fn from(source: ToolboxIdlError) -> Self {
        ToolboxCliError::ToolboxIdl(source)
    }
}

impl From<ParsePubkeyError> for ToolboxCliError {
    fn from(source: ParsePubkeyError) -> Self {
        ToolboxCliError::ParsePubkey(source)
    }
}

impl From<serde_json::Error> for ToolboxCliError {
    fn from(source: serde_json::Error) -> Self {
        ToolboxCliError::SerdeJson(source)
    }
}
