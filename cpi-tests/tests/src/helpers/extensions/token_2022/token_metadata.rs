use {
    crate::helpers::suite::{
        core::{extension::send_tx, App, ProgramId},
        types::{
            addr_to_sol_pubkey, pin_pubkey_to_addr, pin_to_sol_pubkey, AppUser, SolPubkey,
            TestError, TestResult,
        },
    },
    litesvm::types::TransactionMetadata,
    pinocchio::pubkey::Pubkey,
    solana_keypair::Keypair,
    spl_token_2022_interface::extension::{BaseStateWithExtensions, ExtensionType},
    spl_token_metadata_interface::borsh::BorshDeserialize,
};

pub trait Token2022TokenMetadataExtension {
    fn token_2022_try_create_metadata_account(
        &mut self,
        sender: AppUser,
        metadata_keypair: Option<Keypair>,
        metadata: &spl_token_metadata_interface::state::TokenMetadata,
    ) -> TestResult<(TransactionMetadata, Keypair)>;

    fn token_2022_try_create_mint_account_with_metadata(
        &mut self,
        sender: AppUser,
        mint: Option<Keypair>,
        extensions: &[spl_token_2022_interface::extension::ExtensionType],
        metadata: Option<&spl_token_metadata_interface::state::TokenMetadata>,
    ) -> TestResult<(TransactionMetadata, Keypair)>;

    fn token_2022_try_initialize_token_metadata(
        &mut self,
        sender: AppUser,
        metadata_account: &Pubkey,
        update_authority: &Pubkey,
        mint: &Pubkey,
        mint_authority: AppUser,
        name: &str,
        symbol: &str,
        uri: &str,
    ) -> TestResult<TransactionMetadata>;

    fn token_2022_try_initialize_metadata_pointer(
        &mut self,
        sender: AppUser,
        mint: &Pubkey,
        authority: Option<&Pubkey>,
        metadata_address: Option<&Pubkey>,
    ) -> TestResult<TransactionMetadata>;

    // fn token_2022_try_update_token_metadata_field(
    //     &mut self,
    //     sender: AppUser,
    //     metadata_account: &Pubkey,
    //     field: spl_token_metadata_interface::state::Field,
    //     value: &str,
    // ) -> TestResult<TransactionMetadata>;

    // fn token_2022_try_remove_token_metadata_key(
    //     &mut self,
    //     sender: AppUser,
    //     metadata_account: &Pubkey,
    //     key: &str,
    //     id_in_seed: bool,
    // ) -> TestResult<TransactionMetadata>;

    // fn token_2022_try_update_metadata_pointer(
    //     &mut self,
    //     sender: AppUser,
    //     mint: &Pubkey,
    //     metadata_address: Option<&Pubkey>,
    // ) -> TestResult<TransactionMetadata>;

    fn token_2022_query_token_metadata_state(
        &self,
        metadata: &Pubkey,
    ) -> TestResult<spl_token_metadata_interface::state::TokenMetadata>;

    fn token_2022_query_metadata_pointer_state(
        &self,
        mint: &Pubkey,
    ) -> TestResult<spl_token_2022_interface::extension::metadata_pointer::MetadataPointer>;
}

impl Token2022TokenMetadataExtension for App {
    fn token_2022_try_create_metadata_account(
        &mut self,
        sender: AppUser,
        metadata_keypair: Option<Keypair>,
        metadata: &spl_token_metadata_interface::state::TokenMetadata,
    ) -> TestResult<(TransactionMetadata, Keypair)> {
        let ProgramId {
            token_2022_program, ..
        } = self.program_id;

        let account_size = metadata.tlv_size_of().map_err(TestError::from_raw_error)?;

        self.create_account(sender, metadata_keypair, account_size, &token_2022_program)
    }

    fn token_2022_try_create_mint_account_with_metadata(
        &mut self,
        sender: AppUser,
        mint: Option<Keypair>,
        extensions: &[spl_token_2022_interface::extension::ExtensionType],
        metadata: Option<&spl_token_metadata_interface::state::TokenMetadata>,
    ) -> TestResult<(TransactionMetadata, Keypair)> {
        let ProgramId {
            token_2022_program, ..
        } = self.program_id;

        // calculate base size for all other extensions
        let other_extensions: Vec<_> = extensions
            .iter()
            .filter(|&ext| ext != &ExtensionType::TokenMetadata)
            .cloned()
            .collect();

        let base_size = ExtensionType::try_calculate_account_len::<
            spl_token_2022_interface::state::Mint,
        >(&other_extensions)
        .map_err(TestError::from_raw_error)?;

        let account_size = match metadata {
            Some(metadata) => {
                if !extensions.contains(&ExtensionType::TokenMetadata) {
                    Err(TestError::from_raw_error("TokenMetadata isn't found"))?;
                }

                // calculate base size of metadata extensions
                let metadata_tlv_size =
                    metadata.tlv_size_of().map_err(TestError::from_raw_error)?;

                base_size + metadata_tlv_size
            }
            None => base_size,
        };

        self.create_account(sender, mint, account_size, &token_2022_program)
    }

    fn token_2022_try_initialize_token_metadata(
        &mut self,
        sender: AppUser,
        metadata_account: &Pubkey,
        update_authority: &Pubkey,
        mint: &Pubkey,
        mint_authority: AppUser,
        name: &str,
        symbol: &str,
        uri: &str,
    ) -> TestResult<TransactionMetadata> {
        let ProgramId {
            token_2022_program, ..
        } = self.program_id;

        let signers = &[&sender.keypair(), &mint_authority.keypair()];

        let ix = spl_token_metadata_interface::instruction::initialize(
            &token_2022_program.to_bytes().into(),
            &pin_pubkey_to_addr(metadata_account),
            &pin_pubkey_to_addr(update_authority),
            &pin_pubkey_to_addr(mint),
            &mint_authority.pubkey().to_bytes().into(),
            name.to_string(),
            symbol.to_string(),
            uri.to_string(),
        );

        let ix_legacy = solana_instruction::Instruction {
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

        send_tx(
            &mut self.litesvm,
            &[ix_legacy],
            signers,
            self.is_log_displayed,
        )
    }

    fn token_2022_try_initialize_metadata_pointer(
        &mut self,
        sender: AppUser,
        mint: &Pubkey,
        authority: Option<&Pubkey>,
        metadata_address: Option<&Pubkey>,
    ) -> TestResult<TransactionMetadata> {
        let ProgramId {
            token_2022_program, ..
        } = self.program_id;

        let signers = &[&sender.keypair()];

        let ix = spl_token_2022_interface::extension::metadata_pointer::instruction::initialize(
            &token_2022_program.to_bytes().into(),
            &pin_pubkey_to_addr(mint),
            authority.map(pin_pubkey_to_addr),
            metadata_address.map(pin_pubkey_to_addr),
        )
        .map_err(TestError::from_raw_error)?;

        let ix_legacy = solana_instruction::Instruction {
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

        send_tx(
            &mut self.litesvm,
            &[ix_legacy],
            signers,
            self.is_log_displayed,
        )
    }

    fn token_2022_query_token_metadata_state(
        &self,
        metadata: &Pubkey,
    ) -> TestResult<spl_token_metadata_interface::state::TokenMetadata> {
        self.litesvm
            .get_account(&pin_to_sol_pubkey(metadata))
            .map(|x| spl_token_metadata_interface::state::TokenMetadata::try_from_slice(&x.data))
            .transpose()
            .map_err(TestError::from_raw_error)?
            .ok_or(TestError::from_raw_error("The state isn't found"))
    }

    fn token_2022_query_metadata_pointer_state(
        &self,
        mint: &Pubkey,
    ) -> TestResult<spl_token_2022_interface::extension::metadata_pointer::MetadataPointer> {
        let account = self
            .litesvm
            .get_account(&pin_to_sol_pubkey(mint))
            .ok_or(TestError::from_raw_error("The account isn't found"))?;

        // Parse the mint account with extensions
        let mint_with_extensions = spl_token_2022_interface::extension::StateWithExtensions::<
            spl_token_2022_interface::state::Mint,
        >::unpack(&account.data)
        .map_err(TestError::from_raw_error)?;

        // Get the MetadataPointer extension
        mint_with_extensions
            .get_extension::<spl_token_2022_interface::extension::metadata_pointer::MetadataPointer>()
            .map(|&x| x)
            .map_err(TestError::from_raw_error)
    }
}
