use solana_sdk::address_lookup_table::instruction::close_lookup_table;
use solana_sdk::address_lookup_table::instruction::create_lookup_table;
use solana_sdk::address_lookup_table::instruction::extend_lookup_table;
use solana_sdk::address_lookup_table::state::AddressLookupTable;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signature;
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
                        ToolboxEndpointError::Custom(
                            "Address lookup table not found".to_string(),
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
        match self.get_account_data(address_lookup_table).await? {
            Some(data) => {
                let address_lookup_table =
                    AddressLookupTable::deserialize(&data)?;
                eprintln!(
                    "address_lookup_table.meta: {:?}",
                    address_lookup_table.meta
                );
                Ok(Some(address_lookup_table.addresses.to_vec()))
            },
            None => Ok(None),
        }
    }

    pub async fn process_address_lookup_table_new(
        &mut self,
        payer: &Keypair,
        authority: &Pubkey,
    ) -> Result<Pubkey, ToolboxEndpointError> {
        let clock = self.get_sysvar_clock().await?;
        let (instruction, address_lookup_table) =
            create_lookup_table(*authority, payer.pubkey(), clock.epoch);
        self.process_instruction(payer, instruction).await?;
        Ok(address_lookup_table)
    }

    pub async fn process_address_lookup_table_extend(
        &mut self,
        payer: &Keypair,
        authority: &Keypair,
        address_lookup_table: &Pubkey,
        lookup_addresses: Vec<Pubkey>,
    ) -> Result<Signature, ToolboxEndpointError> {
        let instruction = extend_lookup_table(
            *address_lookup_table,
            authority.pubkey(),
            Some(payer.pubkey()),
            lookup_addresses,
        );
        self.process_instruction_with_signers(payer, instruction, &[authority])
            .await
    }

    pub async fn process_address_lookup_table_close(
        &mut self,
        payer: &Keypair,
        authority: &Keypair,
        address_lookup_table: &Pubkey,
        spill: &Pubkey,
    ) -> Result<Signature, ToolboxEndpointError> {
        let instruction = close_lookup_table(
            *address_lookup_table,
            authority.pubkey(),
            *spill,
        );
        self.process_instruction_with_signers(payer, instruction, &[authority])
            .await
    }
}
