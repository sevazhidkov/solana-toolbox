use std::num::TryFromIntError;

use solana_sdk::pubkey::ParsePubkeyError;
use solana_toolbox_endpoint::ToolboxEndpointError;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;

#[derive(Debug)]
pub enum ToolboxIdlError {
    ToolboxEndpoint(ToolboxEndpointError),
    Pubkey(solana_sdk::pubkey::PubkeyError),
    ParsePubkey(solana_sdk::pubkey::ParsePubkeyError),
    TryFromInt(std::num::TryFromIntError),
    Inflate(String),
    FromUtf8(std::string::FromUtf8Error),
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
        breadcrumbs: ToolboxIdlBreadcrumbs,
    },
    InvalidSliceLength {
        offset: usize,
        length: usize,
        breadcrumbs: ToolboxIdlBreadcrumbs,
    },
    InvalidTypeObject {
        breadcrumbs: ToolboxIdlBreadcrumbs,
    },
    InvalidPubkey {
        parsing: ParsePubkeyError,
        breadcrumbs: ToolboxIdlBreadcrumbs,
    },
    InvalidConversionInteger {
        conversion: TryFromIntError,
        breadcrumbs: ToolboxIdlBreadcrumbs,
    },
    Custom {
        failure: String,
        breadcrumbs: ToolboxIdlBreadcrumbs,
    },
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
