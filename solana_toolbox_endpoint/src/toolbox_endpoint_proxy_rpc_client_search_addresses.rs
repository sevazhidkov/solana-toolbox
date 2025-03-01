use std::collections::HashSet;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use solana_account_decoder::UiDataSliceConfig;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcAccountInfoConfig;
use solana_client::rpc_config::RpcProgramAccountsConfig;
use solana_client::rpc_filter::Memcmp;
use solana_client::rpc_filter::MemcmpEncodedBytes;
use solana_client::rpc_filter::RpcFilterType;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_endpoint_error::ToolboxEndpointError;
use crate::toolbox_endpoint_proxy_rpc_client::ToolboxEndpointProxyRpcClient;

impl ToolboxEndpointProxyRpcClient {
    pub(crate) async fn search_addresses_using_rpc(
        rpc_client: &RpcClient,
        program_id: &Pubkey,
        data_len: Option<usize>,
        data_chunks: &[(usize, &[u8])],
    ) -> Result<HashSet<Pubkey>, ToolboxEndpointError> {
        let mut program_accounts_filters = vec![];
        if let Some(data_len) = data_len {
            program_accounts_filters.push(RpcFilterType::DataSize(
                u64::try_from(data_len).unwrap(),
            ));
        }
        for (slice_offset, slice_bytes) in data_chunks {
            program_accounts_filters.push(RpcFilterType::Memcmp(Memcmp::new(
                *slice_offset,
                MemcmpEncodedBytes::Base64(STANDARD.encode(slice_bytes)),
            )));
        }
        Ok(HashSet::from_iter(
            rpc_client
                .get_program_accounts_with_config(
                    program_id,
                    make_program_accounts_config(program_accounts_filters),
                )
                .await?
                .iter()
                .map(|result| result.0),
        ))
    }
}

fn make_account_info_config() -> RpcAccountInfoConfig {
    RpcAccountInfoConfig {
        encoding: None,
        data_slice: Some(UiDataSliceConfig { offset: 0, length: 0 }),
        commitment: None,
        min_context_slot: None,
    }
}

#[cfg(not(feature = "has_sort_results_field"))]
fn make_program_accounts_config(
    program_accounts_filters: Vec<RpcFilterType>
) -> RpcProgramAccountsConfig {
    RpcProgramAccountsConfig {
        filters: Some(program_accounts_filters),
        account_config: make_account_info_config(),
        with_context: None,
    }
}

#[cfg(feature = "has_sort_results_field")]
fn make_program_accounts_config(
    program_accounts_filters: Vec<RpcFilterType>
) -> RpcProgramAccountsConfig {
    RpcProgramAccountsConfig {
        filters: Some(program_accounts_filters),
        account_config: make_account_info_config(),
        with_context: None,
        sort_results: None,
    }
}
