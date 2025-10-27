use {
    crate::helpers::suite::{
        core::{extension::send_tx, App, ProgramId},
        types::{addr_to_sol_pubkey, AppUser, Target, TestResult},
    },
    litesvm::types::TransactionMetadata,
    solana_address::Address,
};

use spl_token_metadata_interface::state::Field;

pub trait Token2022TokenMetadataExtension {
    fn token_2022_try_token_metadata_initialize(
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

    fn token_2022_try_token_metadata_update_field(
        &mut self,
        target: Target,
        sender: AppUser,
        metadata: &Address,
        update_authority: &Address,
        field: Field,
        value: String,
    ) -> TestResult<TransactionMetadata>;

    fn token_2022_try_token_metadata_remove_key(
        &mut self,
        target: Target,
        sender: AppUser,
        metadata: &Address,
        update_authority: &Address,
        key: String,
        idempotent: bool,
    ) -> TestResult<TransactionMetadata>;

    fn token_2022_try_token_metadata_update_authority(
        &mut self,
        target: Target,
        sender: AppUser,
        metadata: &Address,
        current_authority: &Address,
        new_authority: spl_pod::optional_keys::OptionalNonZeroPubkey,
    ) -> TestResult<TransactionMetadata>;

    fn token_2022_try_token_metadata_emit(
        &mut self,
        target: Target,
        sender: AppUser,
        metadata: &Address,
        start: Option<u64>,
        end: Option<u64>,
    ) -> TestResult<TransactionMetadata>;
}

impl Token2022TokenMetadataExtension for App {
    fn token_2022_try_token_metadata_initialize(
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

    // update field
    fn token_2022_try_token_metadata_update_field(
        &mut self,
        target: Target,
        sender: AppUser,
        metadata: &Address,
        update_authority: &Address,
        field: Field,
        value: String,
    ) -> TestResult<TransactionMetadata> {
        let ProgramId {
            token_2022_program,
            token_2022_proxy,
            ..
        } = self.program_id;

        let token_2022_program_addr = Address::new_from_array(token_2022_program.to_bytes());

        let signers = &[sender.keypair()];

        let ix = spl_token_metadata_interface::instruction::update_field(
            &token_2022_program_addr,
            metadata,
            update_authority,
            field,
            value,
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

    // remove key
    fn token_2022_try_token_metadata_remove_key(
        &mut self,
        target: Target,
        sender: AppUser,
        metadata: &Address,
        update_authority: &Address,
        key: String,
        idempotent: bool,
    ) -> TestResult<TransactionMetadata> {
        let ProgramId {
            token_2022_program,
            token_2022_proxy,
            ..
        } = self.program_id;

        let token_2022_program_addr = Address::new_from_array(token_2022_program.to_bytes());

        let signers = &[sender.keypair()];

        let ix = spl_token_metadata_interface::instruction::remove_key(
            &token_2022_program_addr,
            metadata,
            update_authority,
            key,
            idempotent,
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

    // update authority
    fn token_2022_try_token_metadata_update_authority(
        &mut self,
        target: Target,
        sender: AppUser,
        metadata: &Address,
        current_authority: &Address,
        new_authority: spl_pod::optional_keys::OptionalNonZeroPubkey,
    ) -> TestResult<TransactionMetadata> {
        let ProgramId {
            token_2022_program,
            token_2022_proxy,
            ..
        } = self.program_id;

        let token_2022_program_addr = Address::new_from_array(token_2022_program.to_bytes());

        let signers = &[sender.keypair()];

        let ix = spl_token_metadata_interface::instruction::update_authority(
            &token_2022_program_addr,
            metadata,
            current_authority,
            new_authority,
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

    // emit
    fn token_2022_try_token_metadata_emit(
        &mut self,
        target: Target,
        sender: AppUser,
        metadata: &Address,
        start: Option<u64>,
        end: Option<u64>,
    ) -> TestResult<TransactionMetadata> {
        let ProgramId {
            token_2022_program,
            token_2022_proxy,
            ..
        } = self.program_id;

        let token_2022_program_addr = Address::new_from_array(token_2022_program.to_bytes());

        let signers = &[sender.keypair()];

        let ix = spl_token_metadata_interface::instruction::emit(
            &token_2022_program_addr,
            metadata,
            start,
            end,
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
