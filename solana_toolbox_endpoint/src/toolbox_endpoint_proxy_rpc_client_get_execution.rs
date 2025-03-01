use std::collections::HashSet;
use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_request::RpcRequest;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::instruction::Instruction;
use solana_sdk::message::CompileError;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::TransactionError;
use solana_transaction_status::UiTransactionReturnData;

use crate::toolbox_endpoint_error::ToolboxEndpointError;
use crate::toolbox_endpoint_execution::ToolboxEndpointExecution;
use crate::toolbox_endpoint_proxy_rpc_client::ToolboxEndpointProxyRpcClient;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GetTransactionResponse {
    pub transaction: GetTransactionResponseTransaction,
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
    pub return_data: Option<UiTransactionReturnData>,
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
        rpc_client: &RpcClient,
        signature: &Signature,
    ) -> Result<Option<ToolboxEndpointExecution>, ToolboxEndpointError> {
        let response = match rpc_client
            .send::<Option<GetTransactionResponse>>(
                RpcRequest::GetTransaction,
                json!([
                    signature.to_string(),
                    {
                        "commitment": rpc_client.commitment().commitment.to_string(),
                        "encoding": "json",
                        "maxSupportedTransactionVersion": 0,
                    },
                ]),
            )
            .await?
        {
            Some(response) => response,
            None => return Ok(None),
        };
        let header = response.transaction.message.header;
        let static_signatures_count =
            usize::from(header.num_required_signatures);
        let static_readonly_signed_count =
            usize::from(header.num_readonly_signed_accounts);
        let static_readonly_unsigned_count =
            usize::from(header.num_readonly_unsigned_accounts);
        let mut static_accounts = vec![];
        for static_account_key in &response.transaction.message.account_keys {
            static_accounts.push(Pubkey::from_str(static_account_key)?);
        }
        let static_accounts_count = static_accounts.len();
        let mut signers = HashSet::new();
        for static_account_index in 0..static_signatures_count {
            signers.insert(
                *static_accounts
                    .get(static_account_index)
                    .ok_or(CompileError::AccountIndexOverflow)?,
            );
        }
        let mut readonly = HashSet::new();
        for static_account_index in (static_signatures_count
            - static_readonly_signed_count)
            ..static_signatures_count
        {
            readonly.insert(
                *static_accounts
                    .get(static_account_index)
                    .ok_or(CompileError::AccountIndexOverflow)?,
            );
        }
        for static_account_index in (static_accounts_count
            - static_readonly_unsigned_count)
            ..static_accounts_count
        {
            readonly.insert(
                *static_accounts
                    .get(static_account_index)
                    .ok_or(CompileError::AccountIndexOverflow)?,
            );
        }
        let mut loaded_addresses_writable = vec![];
        let mut loaded_addresses_readonly = vec![];
        if let Some(loaded_addresses) = &response.meta.loaded_addresses {
            for loaded_address_writable_key in &loaded_addresses.writable {
                loaded_addresses_writable
                    .push(Pubkey::from_str(loaded_address_writable_key)?);
            }
            for loaded_address_readonly_key in &loaded_addresses.readonly {
                loaded_addresses_readonly
                    .push(Pubkey::from_str(loaded_address_readonly_key)?);
            }
        }
        for loaded_address_readonly in &loaded_addresses_readonly {
            readonly.insert(*loaded_address_readonly);
        }
        let mut all_accounts = vec![];
        all_accounts.append(&mut static_accounts);
        all_accounts.append(&mut loaded_addresses_writable);
        all_accounts.append(&mut loaded_addresses_readonly);
        let mut instructions = vec![];
        for instruction in response.transaction.message.instructions {
            let instruction_program_id = *all_accounts
                .get(usize::from(instruction.program_id_index))
                .ok_or(CompileError::AccountIndexOverflow)?;
            let mut instruction_accounts = vec![];
            for account_index in &instruction.accounts {
                let account = all_accounts
                    .get(usize::from(*account_index))
                    .ok_or(CompileError::AccountIndexOverflow)?;
                let account_is_readonly = readonly.contains(&account);
                let account_is_signer = signers.contains(&account);
                instruction_accounts.push(if account_is_readonly {
                    AccountMeta::new_readonly(*account, account_is_signer)
                } else {
                    AccountMeta::new(*account, account_is_signer)
                });
            }
            instructions.push(Instruction {
                program_id: instruction_program_id,
                accounts: instruction_accounts,
                data: bs58::decode(instruction.data)
                    .into_vec()
                    .map_err(ToolboxEndpointError::Bs58Decode)?,
            });
        }
        Ok(Some(ToolboxEndpointExecution {
            payer: *all_accounts
                .first()
                .ok_or(CompileError::AccountIndexOverflow)?,
            instructions,
            slot: response.slot,
            error: response.meta.err,
            logs: response.meta.log_messages,
            return_data:
                ToolboxEndpointProxyRpcClient::decode_transaction_return_data(
                    response.meta.return_data,
                )?,
            units_consumed: response.meta.compute_units_consumed,
        }))
    }
}
