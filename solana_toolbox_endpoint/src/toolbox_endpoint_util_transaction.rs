use std::collections::HashSet;

use anyhow::anyhow;
use anyhow::Result;
use solana_sdk::hash::Hash;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

use crate::toolbox_endpoint::ToolboxEndpoint;

impl ToolboxEndpoint {
    pub fn compile_transaction(
        payer: &Keypair,
        instructions: &[Instruction],
        signers: &[&Keypair],
        recent_blockhash: Hash,
    ) -> Result<Transaction> {
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
    ) -> Result<(Pubkey, Vec<Instruction>)> {
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

    pub fn verify_transaction_length(transaction: &Transaction) -> Result<()> {
        let transaction_length = bincode::serialize(transaction)?.len();
        let limit_length = ToolboxEndpoint::TRANSACTION_LENGTH_LIMIT;
        if transaction_length > limit_length {
            return Err(anyhow!(
                "Transaction of size {} exceeds the limit of {} bytes",
                transaction_length,
                limit_length
            ));
        }
        Ok(())
    }

    pub fn verify_transaction_signatures(
        transaction: &Transaction,
    ) -> Result<()> {
        let verified_signatures = transaction.verify_with_results();
        let found_signatures = verified_signatures.len();
        let expected_signatures =
            usize::from(transaction.message.header.num_required_signatures);
        if found_signatures != expected_signatures {
            return Err(anyhow!(
                "Transaction has {} signatures, but requires {}",
                found_signatures,
                expected_signatures
            ));
        }
        if !verified_signatures
            .iter()
            .all(|verify_result| *verify_result)
        {
            return Err(anyhow!("Transaction signatures are invalid"));
        }
        Ok(())
    }
}
