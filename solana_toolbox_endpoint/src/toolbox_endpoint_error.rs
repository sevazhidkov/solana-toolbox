use solana_client::client_error::ClientError;
use solana_program_test::BanksClientError;
use solana_sdk::bs58;
use solana_sdk::instruction::InstructionError;
use solana_sdk::message::CompileError;
use solana_sdk::program_error::ProgramError;
use solana_sdk::pubkey::ParsePubkeyError;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::ParseSignatureError;
use solana_sdk::signature::Signature;
use solana_sdk::signer::SignerError;
use solana_sdk::transaction::TransactionError;

#[derive(Debug)]
pub enum ToolboxEndpointError {
    BanksClient(Box<BanksClientError>),
    Client(Box<ClientError>),
    Program(ProgramError),
    Transaction(TransactionError),
    Instruction(InstructionError),
    Compile(CompileError),
    Signer(SignerError),
    ParsePubkey(ParsePubkeyError),
    ParseSignature(ParseSignatureError),
    UnknownSignature(Signature),
    AccountDoesNotExist(Pubkey, String),
    Bincode(bincode::Error),
    Bs58Decode(bs58::decode::Error),
    Base64Decode(base64::DecodeError),
    Io(std::io::Error),
    PodCastError(bytemuck::PodCastError),
    TryFromInt(std::num::TryFromIntError),
    Timeout(&'static str),
    Custom(String),
}

impl From<BanksClientError> for ToolboxEndpointError {
    fn from(source: BanksClientError) -> Self {
        ToolboxEndpointError::BanksClient(Box::new(source))
    }
}

impl From<ClientError> for ToolboxEndpointError {
    fn from(source: ClientError) -> Self {
        ToolboxEndpointError::Client(Box::new(source))
    }
}

impl From<ProgramError> for ToolboxEndpointError {
    fn from(source: ProgramError) -> Self {
        ToolboxEndpointError::Program(source)
    }
}

impl From<TransactionError> for ToolboxEndpointError {
    fn from(source: TransactionError) -> Self {
        ToolboxEndpointError::Transaction(source)
    }
}

impl From<InstructionError> for ToolboxEndpointError {
    fn from(source: InstructionError) -> Self {
        ToolboxEndpointError::Instruction(source)
    }
}

impl From<CompileError> for ToolboxEndpointError {
    fn from(source: CompileError) -> Self {
        ToolboxEndpointError::Compile(source)
    }
}

impl From<SignerError> for ToolboxEndpointError {
    fn from(source: SignerError) -> Self {
        ToolboxEndpointError::Signer(source)
    }
}

impl From<ParsePubkeyError> for ToolboxEndpointError {
    fn from(source: ParsePubkeyError) -> Self {
        ToolboxEndpointError::ParsePubkey(source)
    }
}
