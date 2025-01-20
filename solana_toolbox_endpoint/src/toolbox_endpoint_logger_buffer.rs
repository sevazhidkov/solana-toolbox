use std::sync::atomic::AtomicU32;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;

use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;

use crate::toolbox_endpoint_error::ToolboxEndpointError;
use crate::toolbox_endpoint_logger::ToolboxEndpointLogger;
use crate::toolbox_endpoint_logger::ToolboxEndpointLoggerTransaction;

#[derive(Debug, Clone)]
pub struct ToolboxEndpointLoggerBufferTransaction {
    pub sequencing: u32,
    pub transaction: ToolboxEndpointLoggerTransaction,
    pub signature: Option<Signature>,
}

#[derive(Debug, Clone)]
pub struct ToolboxEndpointLoggerBufferAccount {
    pub sequencing: u32,
    pub address: Pubkey,
    pub account: Option<Account>,
}

#[derive(Debug, Clone, Default)]
pub struct ToolboxEndpointLoggerBuffer {
    sequencing: Arc<AtomicU32>,
    transactions: Arc<RwLock<Vec<ToolboxEndpointLoggerBufferTransaction>>>,
    accounts: Arc<RwLock<Vec<ToolboxEndpointLoggerBufferAccount>>>,
}

impl ToolboxEndpointLoggerBuffer {
    pub fn new() -> ToolboxEndpointLoggerBuffer {
        ToolboxEndpointLoggerBuffer { ..Default::default() }
    }

    pub fn get_transactions(
        &self
    ) -> RwLockReadGuard<Vec<ToolboxEndpointLoggerBufferTransaction>> {
        self.transactions.read().unwrap()
    }

    pub fn get_accounts(
        &self
    ) -> RwLockReadGuard<Vec<ToolboxEndpointLoggerBufferAccount>> {
        self.accounts.read().unwrap()
    }
}

#[async_trait::async_trait]
impl ToolboxEndpointLogger for ToolboxEndpointLoggerBuffer {
    async fn on_transaction(
        &self,
        transaction: &ToolboxEndpointLoggerTransaction,
        result: &Result<Signature, ToolboxEndpointError>,
    ) {
        let sequencing =
            self.sequencing.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let signature = match result {
            Ok(signature) => Some(*signature),
            Err(_) => None,
        };
        self.transactions.write().unwrap().push(
            ToolboxEndpointLoggerBufferTransaction {
                sequencing,
                transaction: transaction.clone(),
                signature,
            },
        );
    }

    async fn on_account(
        &self,
        address: &Pubkey,
        account: &Option<Account>,
    ) {
        let sequencing =
            self.sequencing.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.accounts.write().unwrap().push(
            ToolboxEndpointLoggerBufferAccount {
                sequencing,
                address: *address,
                account: account.clone(),
            },
        );
    }
}
