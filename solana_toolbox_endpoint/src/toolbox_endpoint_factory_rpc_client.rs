use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_proxy::ToolboxEndpointProxy;

impl ToolboxEndpoint {
    pub fn new_rpc_with_url_and_commitment(
        url: &str,
        commitment_config: CommitmentConfig,
    ) -> ToolboxEndpoint {
        RpcClient::new_with_commitment(url.to_string(), commitment_config)
            .into()
    }
}

impl From<RpcClient> for ToolboxEndpoint {
    fn from(rpc_client: RpcClient) -> Self {
        let proxy: Box<dyn ToolboxEndpointProxy> = Box::new(rpc_client);
        ToolboxEndpoint::from(proxy)
    }
}
