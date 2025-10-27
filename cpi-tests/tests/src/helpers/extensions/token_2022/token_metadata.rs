use {
    crate::helpers::suite::{
        core::{extension::send_tx, App, ProgramId},
        types::{addr_to_sol_pubkey, AppUser, Target, TestResult},
    },
    litesvm::types::TransactionMetadata,
    solana_address::Address,
};

pub trait Token2022TokenMetadataExtension {
    fn token_2022_try_initialize_token_metadata(
        &mut self,
        target: Target,
        sender: AppUser,
        metadata: &Address,
        update_authority: &Address,
        mint: &Address,
        mint_authority: &Address,
        name: String,
        symbol: String,
        uri: String,
    ) -> TestResult<TransactionMetadata>;
}

impl Token2022TokenMetadataExtension for App {
    fn token_2022_try_initialize_token_metadata(
        &mut self,
        target: Target,
        sender: AppUser,
        metadata: &Address,
        update_authority: &Address,
        mint: &Address,
        mint_authority: &Address,
        name: String,
        symbol: String,
        uri: String,
    ) -> TestResult<TransactionMetadata> {
        let ProgramId {
            token_2022_program,
            token_2022_proxy,
            ..
        } = self.program_id;

        let token_2022_program_addr = Address::new_from_array(token_2022_program.to_bytes());

        let signers = &[sender.keypair()];

        let ix = spl_token_metadata_interface::instruction::initialize(
            &token_2022_program_addr,
            metadata,
            update_authority,
            mint,
            mint_authority,
            name,
            symbol,
            uri,
        );

        let additional_accounts = [solana_instruction::AccountMeta::new_readonly(
            token_2022_program,
            false,
        )];

        let mut ix_legacy = solana_instruction::Instruction {
            program_id: addr_to_sol_pubkey(&ix.program_id),
            accounts: ix
                .accounts
                .into_iter()
                .map(|x| solana_instruction::AccountMeta {
                    pubkey: addr_to_sol_pubkey(&x.pubkey),
                    is_signer: x.is_signer,
                    is_writable: x.is_writable,
                })
                .collect(),
            data: ix.data,
        };

        if let Target::Proxy = target {
            ix_legacy.program_id = token_2022_proxy;
            ix_legacy.accounts.extend_from_slice(&additional_accounts);
        }

        send_tx(
            &mut self.litesvm,
            &[ix_legacy],
            signers,
            self.is_log_displayed,
        )
    }

    // Add a query function
}
