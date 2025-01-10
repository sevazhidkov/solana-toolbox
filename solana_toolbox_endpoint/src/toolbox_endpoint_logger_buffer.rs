use std::sync::atomic::AtomicU32;
use std::sync::Arc;
use std::sync::RwLock;

use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;

use crate::toolbox_endpoint_error::ToolboxEndpointError;
use crate::toolbox_endpoint_logger::ToolboxEndpointLogger;
use crate::toolbox_endpoint_logger::ToolboxEndpointLoggerTransaction;

#[derive(Debug, Clone)]
pub struct ToolboxEndpointLoggerBufferTransaction {
    pub index: u32,
    pub transaction: ToolboxEndpointLoggerTransaction,
    pub signature: Option<Signature>,
}

#[derive(Debug, Clone)]
pub struct ToolboxEndpointLoggerBufferAccount {
    pub index: u32,
    pub address: Pubkey,
    pub account: Option<Account>,
}

#[derive(Debug, Clone, Default)]
pub struct ToolboxEndpointLoggerBuffer {
    index: Arc<AtomicU32>,
    pub transactions: Arc<RwLock<Vec<ToolboxEndpointLoggerBufferTransaction>>>,
    pub accounts: Arc<RwLock<Vec<ToolboxEndpointLoggerBufferAccount>>>,
}

impl ToolboxEndpointLoggerBuffer {
    pub fn new() -> ToolboxEndpointLoggerBuffer {
        ToolboxEndpointLoggerBuffer { ..Default::default() }
    }
}

#[async_trait::async_trait]
impl ToolboxEndpointLogger for ToolboxEndpointLoggerBuffer {
    async fn on_transaction(
        &self,
        transaction: &ToolboxEndpointLoggerTransaction,
        result: &Result<Signature, ToolboxEndpointError>,
    ) {
        let index =
            self.index.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let signature = match result {
            Ok(signature) => Some(*signature),
            Err(_) => None,
        };
        self.transactions.write().unwrap().push(
            ToolboxEndpointLoggerBufferTransaction {
                index,
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
        let index =
            self.index.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.accounts.write().unwrap().push(
            ToolboxEndpointLoggerBufferAccount {
                index,
                address: *address,
                account: account.clone(),
            },
        );
    }
}
