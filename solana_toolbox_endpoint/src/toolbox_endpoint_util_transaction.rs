use std::collections::HashSet;

use solana_sdk::hash::Hash;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_error::ToolboxEndpointError;

impl ToolboxEndpoint {
    pub async fn compile_transaction(
        payer: &Keypair,
        instructions: &[Instruction],
        signers: &[&Keypair],
        recent_blockhash: Hash,
    ) -> Result<Transaction, ToolboxEndpointError> {
        let mut transaction =
            Transaction::new_with_payer(instructions, Some(&payer.pubkey()));
        let mut needed_signers_pubkeys = HashSet::new();
        for instruction in instructions {
            for instruction_account in &instruction.accounts {
                if instruction_account.is_signer {
                    needed_signers_pubkeys.insert(instruction_account.pubkey);
                }
            }
        }
        let mut signers_pubkeys = HashSet::new();
        let mut signers_keypairs = vec![];
        signers_pubkeys.insert(payer.pubkey());
        signers_keypairs.push(payer);
        for signer_keypair in signers {
            let signer_pubkey = signer_keypair.pubkey();
            if !needed_signers_pubkeys.contains(&signer_pubkey) {
                continue;
            }
            if !signers_pubkeys.contains(&signer_pubkey) {
                signers_pubkeys.insert(signer_pubkey);
                signers_keypairs.push(signer_keypair);
            }
        }
        transaction.partial_sign(&signers_keypairs, recent_blockhash);
        Ok(transaction)
    }

    pub fn decompile_transaction(
        transaction: &Transaction,
    ) -> Result<(Pubkey, Vec<Instruction>), ToolboxEndpointError> {
        let header = &transaction.message.header;
        let addresses = &transaction.message.account_keys;
        let payer = ToolboxEndpoint::decompile_transaction_payer(addresses)?;
        let instructions = ToolboxEndpoint::decompile_transaction_instructions(
            header.num_required_signatures,
            header.num_readonly_signed_accounts,
            header.num_readonly_unsigned_accounts,
            addresses,
            &[],
            &[],
            &transaction.message.instructions,
        )?;
        Ok((payer, instructions))
    }
}
