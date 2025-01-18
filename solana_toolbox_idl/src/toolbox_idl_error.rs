use std::{num::TryFromIntError, string::FromUtf8Error};

use solana_sdk::pubkey::ParsePubkeyError;
use solana_toolbox_endpoint::ToolboxEndpointError;

use crate::toolbox_idl_context::ToolboxIdlContext;

#[derive(Debug)]
pub enum ToolboxIdlError {
    ToolboxEndpoint(ToolboxEndpointError),
    Pubkey(solana_sdk::pubkey::PubkeyError),
    Inflate(String),
    SerdeJson(serde_json::Error),
    InvalidDiscriminator {
        found: u64,
        expected: u64,
    },
    InvalidDataSize {
        found: usize,
        expected: usize,
    },
    InvalidSliceReadAt {
        offset: usize,
        length: usize,
        bytes: usize,
        context: ToolboxIdlContext,
    },
    InvalidSliceLength {
        offset: usize,
        length: usize,
        context: ToolboxIdlContext,
    },
    InvalidPubkey {
        parsing: ParsePubkeyError,
        context: ToolboxIdlContext,
    },
    InvalidString {
        parsing: FromUtf8Error,
        context: ToolboxIdlContext,
    },
    InvalidInteger {
        conversion: TryFromIntError,
        context: ToolboxIdlContext,
    },
    InvalidTypeLeaf {
        context: ToolboxIdlContext,
    },
    Custom {
        failure: String,
        context: ToolboxIdlContext,
    },
}

impl From<ToolboxEndpointError> for ToolboxIdlError {
    fn from(source: ToolboxEndpointError) -> Self {
        ToolboxIdlError::ToolboxEndpoint(source)
    }
}
