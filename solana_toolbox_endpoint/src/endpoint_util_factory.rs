use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program_test::ProgramTest;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;

use crate::endpoint::Endpoint;
use crate::endpoint_inner::EndpointInner;

impl Endpoint {
    pub async fn new_program_test(
        preloaded_programs: &'static [(Pubkey, &str)]
    ) -> Endpoint {
        let mut program_test = ProgramTest::default();
        program_test.prefer_bpf(true);
        for preloaded_program in preloaded_programs {
            program_test.add_program(
                preloaded_program.1,
                preloaded_program.0,
                None,
            );
        }
        let endpoint_inner: Box<dyn EndpointInner> =
            Box::new(program_test.start_with_context().await);
        Endpoint::from(endpoint_inner)
    }

    pub fn new_rpc_client(
        url: String,
        commitment_config: CommitmentConfig,
    ) -> Endpoint {
        let rpc_client = RpcClient::new_with_commitment(url, commitment_config);
        let endpoint_inner: Box<dyn EndpointInner> = Box::new(rpc_client);
        Endpoint::from(endpoint_inner)
    }
}
