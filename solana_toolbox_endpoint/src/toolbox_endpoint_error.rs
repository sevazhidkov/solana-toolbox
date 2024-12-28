#[derive(Debug)]
pub enum ToolboxEndpointError {
    BanksClient(solana_program_test::BanksClientError),
    Client(solana_client::client_error::ClientError),
    Program(solana_sdk::program_error::ProgramError),
    Elapsed(tokio::time::error::Elapsed),
    Io(std::io::Error),
    Signature(String),
    Custom(&'static str),
}
