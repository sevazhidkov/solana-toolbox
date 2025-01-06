use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_inner::ToolboxEndpointInner;

impl ToolboxEndpoint {
    pub fn new_rpc_with_url_and_commitment(
        url: String,
        commitment_config: CommitmentConfig,
    ) -> ToolboxEndpoint {
        RpcClient::new_with_commitment(url, commitment_config).into()
    }
}

impl From<RpcClient> for ToolboxEndpoint {
    fn from(rpc_client: RpcClient) -> Self {
        let endpoint_inner: Box<dyn ToolboxEndpointInner> =
            Box::new(rpc_client);
        ToolboxEndpoint::from(endpoint_inner)
    }
}
