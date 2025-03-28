use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use solana_sdk::bs58;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_error::ToolboxEndpointError;

impl ToolboxEndpoint {
    pub fn encode_base58(data: &[u8]) -> Result<String, ToolboxEndpointError> {
        Ok(bs58::encode(data).into_string())
    }

    pub fn decode_base58(data: &str) -> Result<Vec<u8>, ToolboxEndpointError> {
        bs58::decode(data)
            .into_vec()
            .map_err(ToolboxEndpointError::Bs58Decode)
    }

    pub fn encode_base64(data: &[u8]) -> Result<String, ToolboxEndpointError> {
        Ok(STANDARD.encode(data))
    }

    pub fn decode_base64(data: &str) -> Result<Vec<u8>, ToolboxEndpointError> {
        STANDARD
            .decode(data)
            .map_err(ToolboxEndpointError::Base64Decode)
    }

    // TODO - add base64/base58 sanitizing functions ?
}
