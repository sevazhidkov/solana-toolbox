use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramInstructionAccount {
    pub index: usize,
    pub name: String,
    pub is_writable: bool,
    pub is_signer: bool,
    pub address: Option<Pubkey>,
    pub pda: Option<ToolboxIdlProgramInstructionAccountPda>,
    // TODO - support is_optional ?
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramInstructionAccountPda {
    pub seeds: Vec<ToolboxIdlProgramInstructionAccountPdaBlob>,
    pub program: Option<ToolboxIdlProgramInstructionAccountPdaBlob>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlProgramInstructionAccountPdaBlob {
    Const { bytes: Vec<u8> },
    Arg { path: String },
    Account { path: String },
}
