use solana_sdk::commitment_config::CommitmentConfig;
use solana_transaction_status::UiReturnDataEncoding;
use solana_transaction_status::UiTransactionReturnData;

use crate::toolbox_endpoint_error::ToolboxEndpointError;
use crate::toolbox_endpoint_proxy_rpc_client::ToolboxEndpointProxyRpcClient;
use crate::ToolboxEndpoint;

impl ToolboxEndpointProxyRpcClient {
    pub(crate) fn get_commitment(&self) -> CommitmentConfig {
        self.inner.commitment()
    }

    pub(crate) fn decode_transaction_return_data(
        return_data: Option<UiTransactionReturnData>,
    ) -> Result<Option<Vec<u8>>, ToolboxEndpointError> {
        return_data
            .map(|return_data| {
                let (payload, encoding) = return_data.data;
                if encoding != UiReturnDataEncoding::Base64 {
                    return Err(ToolboxEndpointError::Custom(
                        "Unknown return data encoding".to_string(),
                    ));
                }
                ToolboxEndpoint::decode_base64(&payload)
            })
            .transpose()
    }
}
