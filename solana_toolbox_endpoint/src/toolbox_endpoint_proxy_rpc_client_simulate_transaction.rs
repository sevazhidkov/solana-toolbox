use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::transaction::VersionedTransaction;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_error::ToolboxEndpointError;
use crate::toolbox_endpoint_execution::ToolboxEndpointExecution;
use crate::toolbox_endpoint_proxy_rpc_client::ToolboxEndpointProxyRpcClient;

impl ToolboxEndpointProxyRpcClient {
    pub(crate) async fn simulate_transaction_using_rpc(
        rpc_client: &RpcClient,
        versioned_transaction: VersionedTransaction,
    ) -> Result<ToolboxEndpointExecution, ToolboxEndpointError> {
        let mut resolved_address_lookup_tables = vec![];
        if let Some(address_table_lookups) =
            versioned_transaction.message.address_table_lookups()
        {
            for address_table_lookup in address_table_lookups {
                let address_lookup_table_key = address_table_lookup.account_key;
                let address_lookup_table_addresses =
                    ToolboxEndpoint::parse_address_lookup_table_addresses(
                        &rpc_client
                            .get_account(&address_lookup_table_key)
                            .await?
                            .data,
                    )?;
                resolved_address_lookup_tables.push((
                    address_lookup_table_key,
                    address_lookup_table_addresses,
                ));
            }
        }
        let (payer, instructions) =
            ToolboxEndpoint::decompile_versioned_transaction(
                &versioned_transaction,
                &resolved_address_lookup_tables,
            )?;
        let outcome =
            rpc_client.simulate_transaction(&versioned_transaction).await?;
        Ok(ToolboxEndpointExecution {
            payer,
            instructions,
            slot: outcome.context.slot,
            error: outcome.value.err,
            logs: outcome.value.logs,
            return_data:
                ToolboxEndpointProxyRpcClient::decode_transaction_return_data(
                    outcome.value.return_data,
                )?,
            units_consumed: outcome.value.units_consumed,
        })
    }
}
