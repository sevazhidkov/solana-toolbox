use std::num::ParseIntError;
use std::num::TryFromIntError;
use std::string::FromUtf8Error;

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
        expected: Vec<u8>,
        found: Vec<u8>,
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
    InvalidConstLiteral {
        parsing: ParseIntError,
        context: ToolboxIdlContext,
    },
    Custom {
        failure: String,
        context: ToolboxIdlContext,
    },
    UnknownProgramType {
        name: String,
    },
    UnknownProgramInstruction {
        name: String,
    },
    UnknownProgramAccount {
        name: String,
    },
}

impl From<ToolboxEndpointError> for ToolboxIdlError {
    fn from(source: ToolboxEndpointError) -> Self {
        ToolboxIdlError::ToolboxEndpoint(source)
    }
}
