#[derive(Debug)]
pub enum ToolboxEndpointError {
    BanksClient(solana_program_test::BanksClientError),
    Client(solana_client::client_error::ClientError),
    Program(solana_sdk::program_error::ProgramError),
    Bincode(bincode::Error),
    Io(std::io::Error),
    PodCastError(bytemuck::PodCastError),
    Custom(&'static str),
}
