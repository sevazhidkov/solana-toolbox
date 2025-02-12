use solana_client::client_error::ClientError;
use solana_program_test::BanksClientError;
use solana_sdk::program_error::ProgramError;

#[derive(Debug)]
pub enum ToolboxEndpointError {
    BanksClient(Box<BanksClientError>),
    Client(Box<ClientError>),
    Program(Box<ProgramError>),
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
