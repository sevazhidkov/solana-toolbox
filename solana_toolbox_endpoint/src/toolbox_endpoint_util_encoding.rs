use std::str::FromStr;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use solana_sdk::bs58;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair, Keypair, Signature};

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_error::ToolboxEndpointError;

impl ToolboxEndpoint {
    pub fn encode_base58(data: &[u8]) -> String {
        bs58::encode(data).into_string()
    }

    pub fn encode_base64(data: &[u8]) -> String {
        STANDARD.encode(data)
    }

    pub fn encode_url(data: &str) -> String {
        urlencoding::encode(data).to_string()
    }

    pub fn sanitize_and_decode_base58(
        raw: &str,
    ) -> Result<Vec<u8>, ToolboxEndpointError> {
        let sanitized = raw.replace(|c| !char::is_ascii_alphanumeric(&c), "");
        bs58::decode(sanitized)
            .into_vec()
            .map_err(ToolboxEndpointError::Bs58Decode)
    }

    pub fn sanitize_and_decode_base64(
        raw: &str,
    ) -> Result<Vec<u8>, ToolboxEndpointError> {
        let sanitized = raw.replace(
            |c| {
                !(char::is_ascii_alphanumeric(&c)
                    || c == '+'
                    || c == '/'
                    || c == '=')
            },
            "",
        );
        STANDARD
            .decode(sanitized)
            .map_err(ToolboxEndpointError::Base64Decode)
    }

    pub fn sanitize_and_decode_signature(
        raw: &str,
    ) -> Result<Signature, ToolboxEndpointError> {
        let sanitized = raw.replace(|c| !char::is_ascii_alphanumeric(&c), "");
        Signature::from_str(&sanitized)
            .map_err(ToolboxEndpointError::ParseSignature)
    }

    pub fn sanitize_and_decode_pubkey(
        raw: &str,
    ) -> Result<Pubkey, ToolboxEndpointError> {
        let sanitized = raw.replace(|c| !char::is_ascii_alphanumeric(&c), "");
        Pubkey::from_str(&sanitized).map_err(ToolboxEndpointError::ParsePubkey)
    }

    pub fn sanitize_and_decode_keypair(
        raw: &str,
    ) -> Result<Keypair, ToolboxEndpointError> {
        let sanitized = raw.replace(
            |c| {
                !(char::is_ascii_alphanumeric(&c)
                    || c == '['
                    || c == ']'
                    || c == ',')
            },
            "",
        );
        if sanitized.starts_with("[") {
            return read_keypair(&mut sanitized.as_bytes()).map_err(|err| {
                ToolboxEndpointError::Custom(format!(
                    "Could not read keypair as JSON byte array: {:?}",
                    err
                ))
            });
        }
        Keypair::from_bytes(
            &bs58::decode(sanitized)
                .into_vec()
                .map_err(ToolboxEndpointError::Bs58Decode)?,
        )
        .map_err(|err| {
            ToolboxEndpointError::Custom(format!(
                "Could not read keypair as base58: {:?}",
                err
            ))
        })
    }
}
