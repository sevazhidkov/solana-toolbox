use anyhow::anyhow;
use anyhow::Result;
use solana_sdk::address_lookup_table::instruction::close_lookup_table;
use solana_sdk::address_lookup_table::instruction::create_lookup_table;
use solana_sdk::address_lookup_table::instruction::deactivate_lookup_table;
use solana_sdk::address_lookup_table::instruction::extend_lookup_table;
use solana_sdk::address_lookup_table::instruction::freeze_lookup_table;
use solana_sdk::address_lookup_table::program;
use solana_sdk::address_lookup_table::state::AddressLookupTable;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::toolbox_endpoint::ToolboxEndpoint;

impl ToolboxEndpoint {
    pub const ADDRESS_LOOKUP_TABLE_PROGRAM_ID: Pubkey = program::ID;

    pub async fn resolve_address_lookup_tables(
        &mut self,
        address_lookup_table: &[Pubkey],
    ) -> Result<Vec<(Pubkey, Vec<Pubkey>)>> {
        let mut resolved_address_lookup_tables = vec![];
        for address_lookup_table in address_lookup_table {
            resolved_address_lookup_tables.push((
                *address_lookup_table,
                self.get_address_lookup_table_addresses(address_lookup_table)
                    .await?
                    .ok_or_else(|| {
                        anyhow!(
                            "Could not get account: {} (address lookup table)",
                            address_lookup_table.to_string(),
                        )
                    })?,
            ))
        }
        Ok(resolved_address_lookup_tables)
    }

    pub async fn get_address_lookup_table_addresses(
        &mut self,
        address_lookup_table: &Pubkey,
    ) -> Result<Option<Vec<Pubkey>>> {
        self.get_account_data(address_lookup_table)
            .await?
            .map(|data| {
                ToolboxEndpoint::parse_address_lookup_table_addresses(&data)
            })
            .transpose()
    }

    pub fn parse_address_lookup_table_addresses(
        address_lookup_table_data: &[u8],
    ) -> Result<Vec<Pubkey>> {
        Ok(AddressLookupTable::deserialize(address_lookup_table_data)?
            .addresses
            .to_vec())
    }

    pub async fn process_address_lookup_table_new(
        &mut self,
        payer: &Keypair,
        authority: &Keypair,
        lookup_addresses: &[Pubkey],
    ) -> Result<Pubkey> {
        let slot_hashes = self.get_sysvar_slot_hashes().await?;
        let most_recent_slot = slot_hashes.first().unwrap().0;
        let (instruction, address_lookup_table) = create_lookup_table(
            authority.pubkey(),
            payer.pubkey(),
            most_recent_slot,
        );
        self.process_instruction(payer, instruction).await?;
        self.process_address_lookup_table_extend(
            payer,
            authority,
            &address_lookup_table,
            lookup_addresses,
        )
        .await?;
        Ok(address_lookup_table)
    }

    pub async fn process_address_lookup_table_extend(
        &mut self,
        payer: &Keypair,
        authority: &Keypair,
        address_lookup_table: &Pubkey,
        lookup_addresses: &[Pubkey],
    ) -> Result<()> {
        for lookup_addresses_chunk in lookup_addresses.chunks(27) {
            let instruction = extend_lookup_table(
                *address_lookup_table,
                authority.pubkey(),
                Some(payer.pubkey()),
                lookup_addresses_chunk.to_vec(),
            );
            self.process_instruction_with_signers(
                payer,
                instruction,
                &[authority],
            )
            .await?;
        }
        Ok(())
    }

    pub async fn process_address_lookup_table_freeze(
        &mut self,
        payer: &Keypair,
        authority: &Keypair,
        address_lookup_table: &Pubkey,
    ) -> Result<()> {
        let instruction =
            freeze_lookup_table(*address_lookup_table, authority.pubkey());
        self.process_instruction_with_signers(payer, instruction, &[authority])
            .await?;
        Ok(())
    }

    pub async fn process_address_lookup_table_deactivate(
        &mut self,
        payer: &Keypair,
        authority: &Keypair,
        address_lookup_table: &Pubkey,
    ) -> Result<()> {
        let instruction =
            deactivate_lookup_table(*address_lookup_table, authority.pubkey());
        self.process_instruction_with_signers(payer, instruction, &[authority])
            .await?;
        Ok(())
    }

    pub async fn process_address_lookup_table_close(
        &mut self,
        payer: &Keypair,
        authority: &Keypair,
        address_lookup_table: &Pubkey,
        spill: &Pubkey,
    ) -> Result<()> {
        let instruction = close_lookup_table(
            *address_lookup_table,
            authority.pubkey(),
            *spill,
        );
        self.process_instruction_with_signers(payer, instruction, &[authority])
            .await?;
        Ok(())
    }

    pub async fn process_address_lookup_table_postfix(
        &mut self,
        payer: &Keypair,
        authority: &Keypair,
        address_lookup_table: &Pubkey,
    ) -> Result<()> {
        self.forward_clock_slot(1).await?;
        self.process_address_lookup_table_extend(
            payer,
            authority,
            address_lookup_table,
            &[Pubkey::default()],
        )
        .await?;
        Ok(())
    }
}
