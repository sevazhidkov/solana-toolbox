use std::str::FromStr;

use anyhow::Result;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use solana_sdk::bs58;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signature;

use crate::toolbox_endpoint::ToolboxEndpoint;

impl ToolboxEndpoint {
    pub fn encode_base16(data: &[u8]) -> String {
        let mut bytes = vec![];
        for byte in data {
            bytes.push(format!("{:02X}", byte));
        }
        bytes.join("")
    }

    pub fn encode_base58(data: &[u8]) -> String {
        bs58::encode(data).into_string()
    }

    pub fn encode_base64(data: &[u8]) -> String {
        STANDARD.encode(data)
    }

    pub fn encode_url(data: &str) -> String {
        urlencoding::encode(data).to_string()
    }

    pub fn encode_keypair_base58(keypair: &Keypair) -> String {
        let bytes = &keypair.to_bytes();
        ToolboxEndpoint::encode_base58(bytes)
    }

    pub fn encode_keypair_json_array(keypair: &Keypair) -> String {
        let bytes = keypair.to_bytes().to_vec();
        serde_json::to_string(&bytes).unwrap()
    }

    pub fn sanitize_and_decode_base16(raw: &str) -> Result<Vec<u8>> {
        let sanitized = raw.replace(|c| !char::is_ascii_alphanumeric(&c), "");
        let mut bytes = vec![];
        for byte in 0..(sanitized.len() / 2) {
            let byte_idx = byte * 2;
            let byte_hex = &sanitized[byte_idx..byte_idx + 2];
            bytes.push(u8::from_str_radix(byte_hex, 16)?);
        }
        Ok(bytes)
    }

    pub fn sanitize_and_decode_base58(raw: &str) -> Result<Vec<u8>> {
        let sanitized = raw.replace(|c| !char::is_ascii_alphanumeric(&c), "");
        Ok(bs58::decode(sanitized).into_vec()?)
    }

    pub fn sanitize_and_decode_base64(raw: &str) -> Result<Vec<u8>> {
        let sanitized = raw.replace(
            |c| {
                !(char::is_ascii_alphanumeric(&c)
                    || c == '+'
                    || c == '/'
                    || c == '=')
            },
            "",
        );
        Ok(STANDARD.decode(sanitized)?)
    }

    pub fn sanitize_and_decode_signature(raw: &str) -> Result<Signature> {
        let sanitized = raw.replace(|c| !char::is_ascii_alphanumeric(&c), "");
        Ok(Signature::from_str(&sanitized)?)
    }

    pub fn sanitize_and_decode_pubkey(raw: &str) -> Result<Pubkey> {
        let sanitized = raw.replace(|c| !char::is_ascii_alphanumeric(&c), "");
        Ok(Pubkey::from_str(&sanitized)?)
    }

    pub fn sanitize_and_decode_keypair_json_array(
        raw: &str,
    ) -> Result<Keypair> {
        let sanitized = raw.replace(
            |c| {
                !(char::is_ascii_alphanumeric(&c)
                    || c == '['
                    || c == ']'
                    || c == ',')
            },
            "",
        );
        let decoded = serde_json::from_str::<Vec<u8>>(&sanitized)?;
        Ok(Keypair::from_bytes(&decoded)?)
    }

    pub fn sanitize_and_decode_keypair_base58(raw: &str) -> Result<Keypair> {
        let decoded = ToolboxEndpoint::sanitize_and_decode_base58(raw)?;
        Ok(Keypair::from_bytes(&decoded)?)
    }
}
