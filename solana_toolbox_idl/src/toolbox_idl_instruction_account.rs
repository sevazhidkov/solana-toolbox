use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_path::ToolboxIdlPath;
use crate::toolbox_idl_utils::idl_convert_to_value_name;
use crate::ToolboxIdlTypeFlat;
use crate::ToolboxIdlTypeFull;

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
        value: Value,
        type_flat: ToolboxIdlTypeFlat,
        type_full: ToolboxIdlTypeFull,
    },
    Arg {
        path: ToolboxIdlPath,
        type_flat: ToolboxIdlTypeFlat,
        type_full: ToolboxIdlTypeFull,
    },
    Account {
        path: ToolboxIdlPath,
        instruction_account_name: String,
        account_content_path: ToolboxIdlPath,
        account: Option<String>,
        type_flat: ToolboxIdlTypeFlat,
        type_full: ToolboxIdlTypeFull,
    },
}

impl ToolboxIdlInstructionAccount {
    pub fn sanitize_name(name: &str) -> String {
        idl_convert_to_value_name(name)
    }
}
