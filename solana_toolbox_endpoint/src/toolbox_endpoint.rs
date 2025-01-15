use solana_sdk::account::Account;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::Transaction;

use crate::toolbox_endpoint_error::ToolboxEndpointError;
use crate::toolbox_endpoint_logger::ToolboxEndpointLogger;
use crate::toolbox_endpoint_logger::ToolboxEndpointLoggerInstruction;
use crate::toolbox_endpoint_logger::ToolboxEndpointLoggerTransaction;
use crate::toolbox_endpoint_proxy::ToolboxEndpointProxy;

pub struct ToolboxEndpoint {
    proxy: Box<dyn ToolboxEndpointProxy>,
    loggers: Vec<Box<dyn ToolboxEndpointLogger>>,
}

impl From<Box<dyn ToolboxEndpointProxy>> for ToolboxEndpoint {
    fn from(proxy: Box<dyn ToolboxEndpointProxy>) -> Self {
        Self { proxy, loggers: vec![] }
    }
}

impl ToolboxEndpoint {
    pub fn add_logger(
        &mut self,
        logger: Box<dyn ToolboxEndpointLogger>,
    ) {
        self.loggers.push(logger);
    }

    pub async fn get_latest_blockhash(
        &mut self
    ) -> Result<Hash, ToolboxEndpointError> {
        self.proxy.get_latest_blockhash().await
    }

    pub async fn get_accounts(
        &mut self,
        addresses: &[Pubkey],
    ) -> Result<Vec<Option<Account>>, ToolboxEndpointError> {
        let accounts = self.proxy.get_accounts(addresses).await?;
        for logger in &self.loggers {
            for index in 0..accounts.len() {
                logger.on_account(&addresses[index], &accounts[index]).await;
            }
        }
        Ok(accounts)
    }

    pub async fn process_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<Signature, ToolboxEndpointError> {
        let message = &transaction.message;
        let payer = message.account_keys[0];
        let mut signers = vec![];
        for account_index in 0..message.header.num_required_signatures {
            signers.push(message.account_keys[usize::from(account_index)]);
        }
        let mut instructions = vec![];
        for instruction in &message.instructions {
            let mut accounts = vec![];
            for account_index in &instruction.accounts {
                accounts
                    .push(message.account_keys[usize::from(*account_index)]);
            }
            instructions.push(ToolboxEndpointLoggerInstruction {
                program_id: message.account_keys
                    [usize::from(instruction.program_id_index)],
                accounts,
                data: instruction.data.clone(),
            });
        }
        let result = self.proxy.process_transaction(transaction).await;
        for logger in &self.loggers {
            logger
                .on_transaction(
                    &ToolboxEndpointLoggerTransaction {
                        payer,
                        signers: signers.clone(),
                        instructions: instructions.clone(),
                    },
                    &result,
                )
                .await;
        }
        result
    }

    pub async fn process_airdrop(
        &mut self,
        to: &Pubkey,
        lamports: u64,
    ) -> Result<Signature, ToolboxEndpointError> {
        self.proxy.process_airdrop(to, lamports).await
    }

    pub async fn move_clock_forward(
        &mut self,
        unix_timestamp_delta: u64,
        slot_delta: u64,
    ) -> Result<(), ToolboxEndpointError> {
        self.proxy.move_clock_forward(unix_timestamp_delta, slot_delta).await
    }
}
