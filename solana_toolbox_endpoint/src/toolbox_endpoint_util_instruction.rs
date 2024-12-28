use solana_sdk::instruction::Instruction;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signature;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_error::ToolboxEndpointError;

impl ToolboxEndpoint {
    pub async fn process_instruction(
        &mut self,
        instruction: Instruction,
        payer: &Keypair,
    ) -> Result<Signature, ToolboxEndpointError> {
        self.process_instructions_with_signers(&[instruction], payer, &[]).await
    }

    pub async fn process_instruction_with_signers(
        &mut self,
        instruction: Instruction,
        payer: &Keypair,
        signers: &[&Keypair],
    ) -> Result<Signature, ToolboxEndpointError> {
        self.process_instructions_with_signers(&[instruction], payer, signers)
            .await
    }

    pub async fn process_instructions_with_signers(
        &mut self,
        instructions: &[Instruction],
        payer: &Keypair,
        signers: &[&Keypair],
    ) -> Result<Signature, ToolboxEndpointError> {
        let latest_blockhash = self.get_latest_blockhash().await?;
        let mut transaction: Transaction =
            Transaction::new_with_payer(instructions, Some(&payer.pubkey()));
        let mut keypairs = signers.to_owned();
        keypairs.push(payer);
        transaction.partial_sign(&keypairs, latest_blockhash);
        self.process_transaction(transaction).await
    }
}
