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
        let mut keypairs = signers.to_vec();
        keypairs.push(payer);
        transaction.partial_sign(&keypairs, recent_blockhash);
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
