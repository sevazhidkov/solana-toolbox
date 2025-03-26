use std::io;

use solana_sdk::commitment_config::ParseCommitmentLevelError;
use solana_sdk::pubkey::ParsePubkeyError;
use solana_sdk::signature::ParseSignatureError;
use solana_toolbox_endpoint::ToolboxEndpointError;
use solana_toolbox_idl::ToolboxIdlError;

#[derive(Debug)]
pub enum ToolboxCliError {
    ToolboxEndpoint(ToolboxEndpointError),
    ToolboxIdl(ToolboxIdlError),
    ParsePubkey(ParsePubkeyError),
    ParseSignature(ParseSignatureError),
    ParseCommitmentLevel(ParseCommitmentLevelError),
    SerdeJson(serde_json::Error),
    SerdeHjson(serde_hjson::Error),
    Io(io::Error),
    Custom(String),
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

impl From<ParseSignatureError> for ToolboxCliError {
    fn from(source: ParseSignatureError) -> Self {
        ToolboxCliError::ParseSignature(source)
    }
}

impl From<ParseCommitmentLevelError> for ToolboxCliError {
    fn from(source: ParseCommitmentLevelError) -> Self {
        ToolboxCliError::ParseCommitmentLevel(source)
    }
}

// TODO - could remove this regular json completely ?
impl From<serde_json::Error> for ToolboxCliError {
    fn from(source: serde_json::Error) -> Self {
        ToolboxCliError::SerdeJson(source)
    }
}

impl From<serde_hjson::Error> for ToolboxCliError {
    fn from(source: serde_hjson::Error) -> Self {
        ToolboxCliError::SerdeHjson(source)
    }
}

impl From<io::Error> for ToolboxCliError {
    fn from(source: io::Error) -> Self {
        ToolboxCliError::Io(source)
    }
}
