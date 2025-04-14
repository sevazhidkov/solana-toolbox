use std::collections::HashSet;

use anyhow::Result;
use solana_account_decoder::UiDataSliceConfig;
use solana_client::rpc_config::RpcAccountInfoConfig;
use solana_client::rpc_config::RpcProgramAccountsConfig;
use solana_client::rpc_filter::Memcmp;
use solana_client::rpc_filter::MemcmpEncodedBytes;
use solana_client::rpc_filter::RpcFilterType;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_proxy_rpc_client::ToolboxEndpointProxyRpcClient;

impl ToolboxEndpointProxyRpcClient {
    pub(crate) async fn search_addresses_using_rpc(
        &mut self,
        program_id: &Pubkey,
        data_len: Option<usize>,
        data_chunks: &[(usize, &[u8])],
    ) -> Result<HashSet<Pubkey>> {
        let mut program_accounts_filters = vec![];
        if let Some(data_len) = data_len {
            program_accounts_filters
                .push(RpcFilterType::DataSize(u64::try_from(data_len)?));
        }
        for (slice_offset, slice_bytes) in data_chunks {
            let slice_base64 = ToolboxEndpoint::encode_base64(slice_bytes);
            program_accounts_filters.push(RpcFilterType::Memcmp(Memcmp::new(
                *slice_offset,
                MemcmpEncodedBytes::Base64(slice_base64),
            )));
        }
        Ok(HashSet::from_iter(
            self.inner
                .get_program_accounts_with_config(
                    program_id,
                    make_program_accounts_config(
                        program_accounts_filters,
                        self.get_commitment(),
                    ),
                )
                .await?
                .iter()
                .map(|result| result.0),
        ))
    }
}

fn make_account_info_config(
    commitment: CommitmentConfig,
) -> RpcAccountInfoConfig {
    RpcAccountInfoConfig {
        encoding: None,
        data_slice: Some(UiDataSliceConfig {
            offset: 0,
            length: 0,
        }),
        commitment: Some(commitment),
        min_context_slot: None,
    }
}

#[cfg(not(feature = "has_sort_results_field"))]
fn make_program_accounts_config(
    program_accounts_filters: Vec<RpcFilterType>,
    commitment: CommitmentConfig,
) -> RpcProgramAccountsConfig {
    RpcProgramAccountsConfig {
        filters: Some(program_accounts_filters),
        account_config: make_account_info_config(commitment),
        with_context: None,
    }
}

#[cfg(feature = "has_sort_results_field")]
fn make_program_accounts_config(
    program_accounts_filters: Vec<RpcFilterType>,
    commitment: CommitmentConfig,
) -> RpcProgramAccountsConfig {
    RpcProgramAccountsConfig {
        filters: Some(program_accounts_filters),
        account_config: make_account_info_config(commitment),
        with_context: None,
        sort_results: None,
    }
}
