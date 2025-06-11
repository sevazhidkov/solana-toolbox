use std::time::Duration;
use std::time::SystemTime;

use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use solana_client::rpc_request::RpcRequest;
use solana_sdk::instruction::CompiledInstruction;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::TransactionError;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_execution::ToolboxEndpointExecution;
use crate::toolbox_endpoint_proxy_rpc_client::ToolboxEndpointProxyRpcClient;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GetTransactionResponse {
    pub transaction: GetTransactionResponseTransaction,
    pub block_time: Option<i64>,
    pub slot: u64,
    pub meta: GetTransactionResponseMeta,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GetTransactionResponseTransaction {
    pub message: GetTransactionResponseTransactionMessage,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GetTransactionResponseTransactionMessage {
    pub header: GetTransactionResponseTransactionMessageHeader,
    pub account_keys: Vec<String>,
    pub instructions: Vec<GetTransactionResponseTransactionMessageInstruction>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GetTransactionResponseTransactionMessageHeader {
    pub num_readonly_signed_accounts: u8,
    pub num_readonly_unsigned_accounts: u8,
    pub num_required_signatures: u8,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GetTransactionResponseTransactionMessageInstruction {
    pub program_id_index: u8,
    pub accounts: Vec<u8>,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GetTransactionResponseMeta {
    pub loaded_addresses: Option<GetTransactionResponseMetaLoadedAddresses>,
    pub err: Option<TransactionError>,
    pub log_messages: Option<Vec<String>>,
    pub compute_units_consumed: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GetTransactionResponseMetaLoadedAddresses {
    pub writable: Vec<String>,
    pub readonly: Vec<String>,
}

impl ToolboxEndpointProxyRpcClient {
    pub(crate) async fn get_execution_using_rpc(
        &mut self,
        signature: &Signature,
    ) -> Result<Option<ToolboxEndpointExecution>> {
        let response = match self
            .rpc_client
            .send::<Option<GetTransactionResponse>>(
                RpcRequest::GetTransaction,
                json!([
                    signature.to_string(),
                    {
                        "commitment": self.get_commitment().commitment.to_string(),
                        "maxSupportedTransactionVersion": 0,
                    },
                ]),
            )
            .await?
        {
            Some(response) => response,
            None => return Ok(None),
        };
        let mut static_addresses = vec![];
        for static_address in &response.transaction.message.account_keys {
            static_addresses.push(ToolboxEndpoint::sanitize_and_decode_pubkey(
                static_address,
            )?);
        }
        let payer =
            ToolboxEndpoint::decompile_transaction_payer(&static_addresses)?;
        let header = response.transaction.message.header;
        let mut loaded_writable_addresses = vec![];
        let mut loaded_readonly_addresses = vec![];
        if let Some(loaded_addresses) = &response.meta.loaded_addresses {
            for loaded_writable_key in &loaded_addresses.writable {
                loaded_writable_addresses.push(
                    ToolboxEndpoint::sanitize_and_decode_pubkey(
                        loaded_writable_key,
                    )?,
                );
            }
            for loaded_readonly_key in &loaded_addresses.readonly {
                loaded_readonly_addresses.push(
                    ToolboxEndpoint::sanitize_and_decode_pubkey(
                        loaded_readonly_key,
                    )?,
                );
            }
        }
        let mut compiled_instructions = vec![];
        for response_instruction in response.transaction.message.instructions {
            compiled_instructions.push(CompiledInstruction {
                program_id_index: response_instruction.program_id_index,
                accounts: response_instruction.accounts,
                data: ToolboxEndpoint::sanitize_and_decode_base58(
                    &response_instruction.data,
                )?,
            });
        }
        let instructions = ToolboxEndpoint::decompile_transaction_instructions(
            header.num_required_signatures,
            header.num_readonly_signed_accounts,
            header.num_readonly_unsigned_accounts,
            &static_addresses,
            &loaded_writable_addresses,
            &loaded_readonly_addresses,
            &compiled_instructions,
        )?;
        Ok(Some(ToolboxEndpointExecution {
            processed_time: response.block_time.map(|block_time| {
                SystemTime::UNIX_EPOCH + Duration::from_secs(block_time as u64)
            }),
            slot: response.slot,
            payer,
            instructions,
            error: response.meta.err,
            steps: response
                .meta
                .log_messages
                .as_ref()
                .map(|logs| ToolboxEndpointExecution::try_parse_steps(logs))
                .transpose()?,
            logs: response.meta.log_messages,
            units_consumed: response.meta.compute_units_consumed,
        }))
    }
}
