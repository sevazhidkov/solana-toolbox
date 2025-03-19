use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlInstructionAccount {
    pub name: String,
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
