use solana_sdk::address_lookup_table::instruction::close_lookup_table;
use solana_sdk::address_lookup_table::instruction::create_lookup_table;
use solana_sdk::address_lookup_table::instruction::deactivate_lookup_table;
use solana_sdk::address_lookup_table::instruction::extend_lookup_table;
use solana_sdk::address_lookup_table::instruction::freeze_lookup_table;
use solana_sdk::address_lookup_table::state::AddressLookupTable;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_error::ToolboxEndpointError;

impl ToolboxEndpoint {
    pub async fn resolve_address_lookup_tables(
        &mut self,
        address_lookup_table: &[Pubkey],
    ) -> Result<Vec<(Pubkey, Vec<Pubkey>)>, ToolboxEndpointError> {
        let mut resolved_address_lookup_tables = vec![];
        for address_lookup_table in address_lookup_table {
            resolved_address_lookup_tables.push((
                *address_lookup_table,
                self.get_address_lookup_table_addresses(address_lookup_table)
                    .await?
                    .ok_or_else(|| {
                        ToolboxEndpointError::AccountDoesNotExist(
                            *address_lookup_table,
                            "Address Lookup Table".to_string(),
                        )
                    })?,
            ))
        }
        Ok(resolved_address_lookup_tables)
    }

    pub async fn get_address_lookup_table_addresses(
        &mut self,
        address_lookup_table: &Pubkey,
    ) -> Result<Option<Vec<Pubkey>>, ToolboxEndpointError> {
        self.get_account_data(address_lookup_table)
            .await?
            .map(|data| {
                ToolboxEndpoint::parse_address_lookup_table_addresses(&data)
            })
            .transpose()
    }

    pub fn parse_address_lookup_table_addresses(
        address_lookup_table_data: &[u8]
    ) -> Result<Vec<Pubkey>, ToolboxEndpointError> {
        Ok(AddressLookupTable::deserialize(&address_lookup_table_data)?
            .addresses
            .to_vec())
    }

    pub async fn process_address_lookup_table_new(
        &mut self,
        payer: &Keypair,
        authority: &Keypair,
        lookup_addresses: &[Pubkey],
    ) -> Result<Pubkey, ToolboxEndpointError> {
        let slot_hashes = self.get_sysvar_slot_hashes().await?;
        let most_recent_slot = slot_hashes.first().unwrap().0;
        let (instruction, address_lookup_table) = create_lookup_table(
            authority.pubkey(),
            payer.pubkey(),
            most_recent_slot,
        );
        self.process_instruction(payer, instruction).await?;
        self.process_address_lookup_table_extend(
            &payer,
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
    ) -> Result<(), ToolboxEndpointError> {
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
    ) -> Result<(), ToolboxEndpointError> {
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
    ) -> Result<(), ToolboxEndpointError> {
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
    ) -> Result<(), ToolboxEndpointError> {
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
    ) -> Result<(), ToolboxEndpointError> {
        self.forward_clock_slot(1).await?;
        self.process_address_lookup_table_extend(
            &payer,
            &authority,
            address_lookup_table,
            &[Pubkey::default()],
        )
        .await?;
        Ok(())
    }
}
