use std::collections::HashSet;

use solana_sdk::account::Account;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::Transaction;

use crate::toolbox_endpoint_error::ToolboxEndpointError;

#[derive(Debug, Clone)]
pub struct ToolboxEndpointLoggerTransaction {
    pub payer: Pubkey,
    pub signers: Vec<Pubkey>,
    pub instructions: Vec<ToolboxEndpointLoggerInstruction>,
}

#[derive(Debug, Clone)]
pub struct ToolboxEndpointLoggerInstruction {
    pub program_id: Pubkey,
    pub accounts: Vec<AccountMeta>,
    pub data: Vec<u8>,
}

#[async_trait::async_trait]
pub trait ToolboxEndpointLogger {
    async fn on_transaction(
        &self,
        transaction: &ToolboxEndpointLoggerTransaction, // TODO - could be printable directly
        result: &Result<Signature, ToolboxEndpointError>,
    );

    async fn on_account(
        &self,
        address: &Pubkey,
        account: &Option<Account>,
    );
}

impl From<&Transaction> for ToolboxEndpointLoggerTransaction {
    fn from(transaction: &Transaction) -> ToolboxEndpointLoggerTransaction {
        let message = &transaction.message;
        let num_signatures =
            usize::from(message.header.num_required_signatures);
        let num_accounts = message.account_keys.len();
        let payer = message.account_keys[0];
        let mut signers = HashSet::new();
        for account_index in 0..num_signatures {
            signers.insert(message.account_keys[usize::from(account_index)]);
        }
        let mut readonly = HashSet::new();
        for account_index in (num_signatures
            - usize::from(message.header.num_readonly_signed_accounts))
            ..num_signatures
        {
            readonly.insert(message.account_keys[usize::from(account_index)]);
        }
        for account_index in (num_accounts
            - usize::from(message.header.num_readonly_unsigned_accounts))
            ..num_accounts
        {
            readonly.insert(message.account_keys[usize::from(account_index)]);
        }
        let mut instructions = vec![];
        for instruction in &message.instructions {
            let mut accounts = vec![];
            for account_index in &instruction.accounts {
                let account_address =
                    message.account_keys[usize::from(*account_index)];
                let account_is_readonly = readonly.contains(&account_address);
                let account_is_signer = signers.contains(&account_address);
                accounts.push(if account_is_readonly {
                    AccountMeta::new_readonly(
                        account_address,
                        account_is_signer,
                    )
                } else {
                    AccountMeta::new(account_address, account_is_signer)
                });
            }
            instructions.push(ToolboxEndpointLoggerInstruction {
                program_id: message.account_keys
                    [usize::from(instruction.program_id_index)],
                accounts,
                data: instruction.data.clone(),
            });
        }
        ToolboxEndpointLoggerTransaction {
            payer,
            signers: signers.into_iter().collect(),
            instructions: instructions.clone(),
        }
    }
}
