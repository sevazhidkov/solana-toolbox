use std::cmp::min;

use anyhow::Result;
use solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_proxy_rpc_client::ToolboxEndpointProxyRpcClient;

// TODO (FAR) - should it return the first signature "start_before" in results ?
impl ToolboxEndpointProxyRpcClient {
    pub(crate) async fn search_signatures_using_rpc(
        &mut self,
        address: &Pubkey,
        start_before: Option<Signature>,
        rewind_until: Option<Signature>,
        limit: usize,
    ) -> Result<Vec<Signature>> {
        let mut oldest_known_signature = start_before;
        let mut ordered_signatures = vec![];
        let mut retries = 0;
        loop {
            let batch_size = min(
                1000,
                match rewind_until {
                    None => limit,
                    Some(_) => match retries {
                        0 => 10,
                        1 => 100,
                        _ => usize::MAX,
                    },
                },
            );
            retries += 1;
            let signatures = self
                .rpc_client
                .get_signatures_for_address_with_config(
                    address,
                    GetConfirmedSignaturesForAddress2Config {
                        before: oldest_known_signature,
                        until: None,
                        limit: Some(batch_size),
                        commitment: Some(self.get_commitment()),
                    },
                )
                .await?;
            if signatures.is_empty() {
                return Ok(ordered_signatures);
            }
            for signature in &signatures {
                let found_signature =
                    ToolboxEndpoint::sanitize_and_decode_signature(
                        &signature.signature,
                    )?;
                ordered_signatures.push(found_signature);
                if ordered_signatures.len() >= limit {
                    return Ok(ordered_signatures);
                }
                if let Some(rewind_until) = rewind_until {
                    if found_signature == rewind_until {
                        return Ok(ordered_signatures);
                    }
                }
                oldest_known_signature = Some(found_signature);
            }
        }
    }
}
