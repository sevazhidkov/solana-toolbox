use solana_toolbox_endpoint::ToolboxEndpointError;

use crate::toolbox_idl::ToolboxIdl;

#[derive(Debug)]
pub enum ToolboxIdlError {
    ToolboxEndpoint(ToolboxEndpointError),
    Pubkey(solana_sdk::pubkey::PubkeyError),
    ParsePubkey(solana_sdk::pubkey::ParsePubkeyError),
    TryFromInt(std::num::TryFromIntError),
    Inflate(String),
    FromUtf8(std::string::FromUtf8Error),
    SerdeJson(serde_json::Error),
    Overflow(),
    Custom(String), // TODO - use special error type
}

impl From<ToolboxEndpointError> for ToolboxIdlError {
    fn from(source: ToolboxEndpointError) -> Self {
        ToolboxIdlError::ToolboxEndpoint(source)
    }
}

impl ToolboxIdl {
    pub fn get_error_name(
        &self,
        error_code: u64,
    ) -> Option<&String> {
        for (idl_error_name, idl_error_code) in self.errors_codes.iter() {
            if let Some(code) = idl_error_code.as_u64() {
                if error_code == code {
                    return Some(idl_error_name);
                }
            }
        }
        None
    }
}
