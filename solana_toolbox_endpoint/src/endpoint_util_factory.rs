use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program_test::ProgramTestContext;

use crate::endpoint::Endpoint;
use crate::endpoint_inner::EndpointInner;

impl From<ProgramTestContext> for Endpoint {
    fn from(program_test_context: ProgramTestContext) -> Self {
        let endpoint_inner: Box<dyn EndpointInner> =
            Box::new(program_test_context);
        Endpoint::from(endpoint_inner)
    }
}

impl From<RpcClient> for Endpoint {
    fn from(rpc_client: RpcClient) -> Self {
        let endpoint_inner: Box<dyn EndpointInner> = Box::new(rpc_client);
        Endpoint::from(endpoint_inner)
    }
}
