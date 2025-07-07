use anyhow::Result;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use solana_sdk::bs58;

pub fn encode_base16_bytes(data: &[u8]) -> Vec<String> {
    data.iter().map(|b| format!("{:02X}", b)).collect()
}

pub fn encode_base16(data: &[u8]) -> String {
    encode_base16_bytes(data).join("")
}

pub fn encode_base58(data: &[u8]) -> String {
    bs58::encode(data).into_string()
}

pub fn encode_base64(data: &[u8]) -> String {
    STANDARD.encode(data)
}

pub fn sanitize_and_decode_base16(raw: &str) -> Result<Vec<u8>> {
    let sanitized = raw.replace(|c: char| !c.is_ascii_alphanumeric(), "");
    let mut bytes = Vec::new();
    for idx in 0..(sanitized.len() / 2) {
        let slice = &sanitized[idx * 2..idx * 2 + 2];
        bytes.push(u8::from_str_radix(slice, 16)?);
    }
    Ok(bytes)
}

pub fn sanitize_and_decode_base58(raw: &str) -> Result<Vec<u8>> {
    let sanitized = raw.replace(|c: char| !c.is_ascii_alphanumeric(), "");
    Ok(bs58::decode(sanitized).into_vec()?)
}

pub fn sanitize_and_decode_base64(raw: &str) -> Result<Vec<u8>> {
    let sanitized = raw.replace(
        |c: char| {
            !(c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=')
        },
        "",
    );
    Ok(STANDARD.decode(sanitized)?)
}
