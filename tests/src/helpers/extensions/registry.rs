use {
    crate::helpers::suite::{
        core::{extension::send_tx, App, ProgramId},
        types::{addr_to_sol_pubkey, pin_pubkey_to_addr, AppUser, TestError, TestResult},
    },
    litesvm::types::TransactionMetadata,
    pinocchio::pubkey::Pubkey,
};

pub trait TokenExtension {
    fn token_try_initialize_mint(
        &mut self,
        sender: AppUser,
        mint: &Pubkey,
        decimals: u8,
        mint_authority: &Pubkey,
        freeze_authority: Option<&Pubkey>,
    ) -> TestResult<TransactionMetadata>;

    // fn token_query_config(&self) -> TestResult<Config>;
}

impl TokenExtension for App {
    fn token_try_initialize_mint(
        &mut self,
        sender: AppUser,
        mint: &Pubkey,
        decimals: u8,
        mint_authority: &Pubkey,
        freeze_authority: Option<&Pubkey>,
    ) -> TestResult<TransactionMetadata> {
        // programs
        let ProgramId {
            token_2022_program,
            token_2022_caller,
            ..
        } = self.program_id;

        // signers
        let signers = &[sender.keypair()];

        // instruction builder checks program_id, token_2022_program must be specified here
        let ix = spl_token_2022_interface::instruction::initialize_mint(
            &pin_pubkey_to_addr(&token_2022_program.to_bytes()),
            &pin_pubkey_to_addr(&mint),
            &pin_pubkey_to_addr(&mint_authority),
            freeze_authority.map(pin_pubkey_to_addr).as_ref(),
            decimals,
        )
        .map_err(TestError::from_raw_error)?;

        // convert Instruction v3.0.0 to Instruction v2.3.0
        // program_id should be replaced now: token_2022_program -> token_2022_caller
        let ix_legacy = solana_instruction::Instruction {
            program_id: token_2022_caller,
            accounts: ix
                .accounts
                .iter()
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

    // fn token_query_config(&self) -> TestResult<Config> {
    //     get_data(&self.litesvm, &self.pda.token_config())
    // }
}
