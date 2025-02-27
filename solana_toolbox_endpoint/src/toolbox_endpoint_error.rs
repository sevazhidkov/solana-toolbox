use solana_client::client_error::ClientError;
use solana_program_test::BanksClientError;
use solana_sdk::instruction::InstructionError;
use solana_sdk::message::CompileError;
use solana_sdk::program_error::ProgramError;
use solana_sdk::signature::ParseSignatureError;
use solana_sdk::signature::Signature;
use solana_sdk::signer::SignerError;

#[derive(Debug)]
pub enum ToolboxEndpointError {
    BanksClient(Box<BanksClientError>),
    Client(Box<ClientError>),
    Program(Box<ProgramError>),
    Instruction(Box<InstructionError>),
    Compile(Box<CompileError>),
    Signer(Box<SignerError>),
    ParseSignature(ParseSignatureError),
    UnknownSignature(Signature),
    Bincode(bincode::Error),
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
        ToolboxEndpointError::Program(Box::new(source))
    }
}

impl From<InstructionError> for ToolboxEndpointError {
    fn from(source: InstructionError) -> Self {
        ToolboxEndpointError::Instruction(Box::new(source))
    }
}

impl From<CompileError> for ToolboxEndpointError {
    fn from(source: CompileError) -> Self {
        ToolboxEndpointError::Compile(Box::new(source))
    }
}

impl From<SignerError> for ToolboxEndpointError {
    fn from(source: SignerError) -> Self {
        ToolboxEndpointError::Signer(Box::new(source))
    }
}
