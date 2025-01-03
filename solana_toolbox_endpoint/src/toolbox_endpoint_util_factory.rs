use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program_runtime::invoke_context::BuiltinFunctionWithContext;
use solana_program_test::ProgramTest;
use solana_program_test::ProgramTestContext;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_inner::ToolboxEndpointInner;

impl From<ProgramTestContext> for ToolboxEndpoint {
    fn from(program_test_context: ProgramTestContext) -> Self {
        let endpoint_inner: Box<dyn ToolboxEndpointInner> =
            Box::new(program_test_context);
        ToolboxEndpoint::from(endpoint_inner)
    }
}

impl From<RpcClient> for ToolboxEndpoint {
    fn from(rpc_client: RpcClient) -> Self {
        let endpoint_inner: Box<dyn ToolboxEndpointInner> =
            Box::new(rpc_client);
        ToolboxEndpoint::from(endpoint_inner)
    }
}

impl ToolboxEndpoint {
    pub async fn new_program_test_with_preloaded_programs(
        preloaded_programs: &[(
            Pubkey,
            &'static str,
            Option<BuiltinFunctionWithContext>,
        )]
    ) -> ToolboxEndpoint {
        let mut program_test = ProgramTest::default();
        program_test.prefer_bpf(true);
        for preloaded_program in preloaded_programs {
            program_test.add_program(
                preloaded_program.1,
                preloaded_program.0,
                preloaded_program.2,
            );
        }
        program_test.start_with_context().await.into()
    }

    pub fn new_rpc_with_url_and_commitment(
        url: String,
        commitment_config: CommitmentConfig,
    ) -> ToolboxEndpoint {
        RpcClient::new_with_commitment(url, commitment_config).into()
    }
}
