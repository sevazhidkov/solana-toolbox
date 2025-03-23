use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_utils::idl_convert_to_value_name;

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
    Const { bytes: Vec<u8> },
    Arg { path: String },
    Account { path: String },
}

impl ToolboxIdlInstructionAccountPdaBlob {
    pub fn bytes(&self) -> Option<&[u8]> {
        match self {
            ToolboxIdlInstructionAccountPdaBlob::Const { bytes } => Some(bytes),
            ToolboxIdlInstructionAccountPdaBlob::Arg { .. } => None,
            ToolboxIdlInstructionAccountPdaBlob::Account { .. } => None,
        }
    }

    pub fn info(&self) -> Option<(&str, &str)> {
        match self {
            ToolboxIdlInstructionAccountPdaBlob::Const { .. } => None,
            ToolboxIdlInstructionAccountPdaBlob::Arg { path } => {
                Some(("arg", path))
            },
            ToolboxIdlInstructionAccountPdaBlob::Account { path } => {
                Some(("account", path))
            },
        }
    }
}

impl ToolboxIdlInstructionAccount {
    pub fn sanitize_name(name: &str) -> String {
        idl_convert_to_value_name(name)
    }
}
