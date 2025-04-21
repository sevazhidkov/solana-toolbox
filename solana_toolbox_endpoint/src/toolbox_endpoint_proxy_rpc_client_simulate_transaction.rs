use anyhow::anyhow;
use anyhow::Result;
use solana_client::rpc_config::RpcSimulateTransactionConfig;
use solana_sdk::transaction::VersionedTransaction;
use solana_transaction_status::UiTransactionEncoding;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_execution::ToolboxEndpointExecution;
use crate::toolbox_endpoint_proxy::ToolboxEndpointProxy;
use crate::toolbox_endpoint_proxy_rpc_client::ToolboxEndpointProxyRpcClient;

impl ToolboxEndpointProxyRpcClient {
    pub(crate) async fn simulate_transaction_using_rpc(
        &mut self,
        versioned_transaction: VersionedTransaction,
        verify_signatures: bool,
    ) -> Result<ToolboxEndpointExecution> {
        let mut resolved_address_lookup_tables = vec![];
        if let Some(address_table_lookups) =
            versioned_transaction.message.address_table_lookups()
        {
            for address_table_lookup in address_table_lookups {
                let address_lookup_table_key = address_table_lookup.account_key;
                let address_lookup_table_addresses =
                    ToolboxEndpoint::parse_address_lookup_table_addresses(
                        &self
                            .get_account(&address_lookup_table_key)
                            .await?
                            .ok_or_else(|| {
                                anyhow!(
                                    "Could not get account: {} (address lookup table)",
                                    address_lookup_table_key,
                                )
                            })?
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
        let outcome = self
            .inner
            .simulate_transaction_with_config(
                &versioned_transaction,
                RpcSimulateTransactionConfig {
                    sig_verify: verify_signatures,
                    replace_recent_blockhash: false,
                    commitment: Some(self.get_commitment()),
                    encoding: Some(UiTransactionEncoding::Base64),
                    accounts: None,
                    min_context_slot: None,
                    inner_instructions: false,
                },
            )
            .await?;
        Ok(ToolboxEndpointExecution {
            payer,
            instructions,
            slot: outcome.context.slot,
            error: outcome.value.err,
            steps: outcome
                .value
                .logs
                .as_ref()
                .map(|logs| ToolboxEndpointExecution::try_parse_steps(logs))
                .transpose()?,
            logs: outcome.value.logs,
            return_data:
                ToolboxEndpointProxyRpcClient::decode_transaction_return_data(
                    outcome.value.return_data,
                )?,
            units_consumed: outcome.value.units_consumed,
        })
    }
}
