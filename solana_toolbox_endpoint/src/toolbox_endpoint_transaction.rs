use std::collections::HashSet;

use solana_sdk::instruction::AccountMeta;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::Transaction;

#[derive(Debug, Clone)]
pub struct ToolboxEndpointTransaction {
    pub payer: Pubkey,
    pub signers: Vec<Pubkey>,
    pub instructions: Vec<Instruction>,
}

impl From<&Transaction> for ToolboxEndpointTransaction {
    fn from(transaction: &Transaction) -> ToolboxEndpointTransaction {
        let message = &transaction.message;
        let num_signatures =
            usize::from(message.header.num_required_signatures);
        let num_accounts = message.account_keys.len();
        let payer = message.account_keys[0];
        let mut signers = HashSet::new();
        for account_index in 0..num_signatures {
            signers.insert(message.account_keys[account_index]);
        }
        let mut readonly = HashSet::new();
        for account_index in (num_signatures
            - usize::from(message.header.num_readonly_signed_accounts))
            ..num_signatures
        {
            readonly.insert(message.account_keys[account_index]);
        }
        for account_index in (num_accounts
            - usize::from(message.header.num_readonly_unsigned_accounts))
            ..num_accounts
        {
            readonly.insert(message.account_keys[account_index]);
        }
        let mut instructions = vec![];
        for instruction in &message.instructions {
            let mut accounts = vec![];
            for account_index in &instruction.accounts {
                let account_address =
                    message.account_keys[usize::from(*account_index)];
                let account_is_readonly = readonly.contains(&account_address);
                let account_is_signer = signers.contains(&account_address);
                accounts.push(
                    if account_is_readonly {
                        AccountMeta::new_readonly(
                            account_address,
                            account_is_signer,
                        )
                    } else {
                        AccountMeta::new(account_address, account_is_signer)
                    },
                );
            }
            instructions.push(Instruction {
                program_id: message.account_keys
                    [usize::from(instruction.program_id_index)],
                accounts,
                data: instruction.data.clone(),
            });
        }
        ToolboxEndpointTransaction {
            payer,
            signers: signers.into_iter().collect(),
            instructions,
        }
    }
}
