use std::sync::Arc;

use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::{
    toolbox_idl_path::ToolboxIdlPath,
    toolbox_idl_utils::idl_convert_to_value_name, ToolboxIdlAccount,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlInstructionAccount {
    pub name: String,
    pub docs: Option<Value>,
    pub writable: bool,
    pub signer: bool,
    pub optional: bool,
    pub address: Option<Pubkey>,
    pub pda: Option<ToolboxIdlInstructionAccountPda>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlInstructionAccountPda {
    pub seeds: Vec<ToolboxIdlInstructionAccountPdaBlob>,
    pub program: Option<ToolboxIdlInstructionAccountPdaBlob>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlInstructionAccountPdaBlob {
    Const {
        bytes: Vec<u8>,
    },
    Arg {
        path: ToolboxIdlPath,
    },
    Account {
        path: ToolboxIdlPath,
        account: Option<Arc<ToolboxIdlAccount>>,
    },
}

impl ToolboxIdlInstructionAccountPdaBlob {
    pub fn bytes(&self) -> Option<&[u8]> {
        match self {
            ToolboxIdlInstructionAccountPdaBlob::Const { bytes } => Some(bytes),
            ToolboxIdlInstructionAccountPdaBlob::Arg { .. } => None,
            ToolboxIdlInstructionAccountPdaBlob::Account { .. } => None,
        }
    }

    pub fn need(&self) -> Option<(&str, String)> {
        match self {
            ToolboxIdlInstructionAccountPdaBlob::Const { .. } => None,
            ToolboxIdlInstructionAccountPdaBlob::Arg { path } => {
                Some(("arg", path.export()))
            },
            ToolboxIdlInstructionAccountPdaBlob::Account { path, .. } => {
                Some(("account", path.export()))
            },
        }
    }
}

impl ToolboxIdlInstructionAccount {
    pub fn sanitize_name(name: &str) -> String {
        idl_convert_to_value_name(name)
    }
}
