use anyhow::anyhow;
use anyhow::Result;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_transaction_status::UiReturnDataEncoding;
use solana_transaction_status::UiTransactionReturnData;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_proxy_rpc_client::ToolboxEndpointProxyRpcClient;

impl ToolboxEndpointProxyRpcClient {
    pub(crate) fn get_commitment(&self) -> CommitmentConfig {
        self.rpc_client.commitment()
    }

    pub(crate) fn decode_transaction_return_data(
        return_data: Option<UiTransactionReturnData>,
    ) -> Result<Option<Vec<u8>>> {
        return_data
            .map(|return_data| {
                let (payload, encoding) = return_data.data;
                if encoding != UiReturnDataEncoding::Base64 {
                    return Err(anyhow!(
                        "Unknown transaction return data encoding: {:?}",
                        encoding
                    ));
                }
                ToolboxEndpoint::sanitize_and_decode_base64(&payload)
            })
            .transpose()
    }
}
