use {
    crate::helpers::suite::{
        core::{
            extension::{get_data, send_tx_with_ix},
            App, ProgramId,
        },
        types::{AppUser, SolPubkey, TestError, TestResult},
    },
    litesvm::types::TransactionMetadata,
};

pub trait TokenExtension {
    // fn token_try_init(
    //     &mut self,
    //     sender: AppUser,
    //     rotation_timeout: Option<u32>,
    //     account_registration_fee: Option<AssetItem>,
    //     account_data_size_range: Option<Range>,
    // ) -> TestResult<TransactionMetadata>;

    // fn token_query_config(&self) -> TestResult<Config>;
}

impl TokenExtension for App {
    // fn token_try_init(
    //     &mut self,
    //     sender: AppUser,
    //     rotation_timeout: Option<u32>,
    //     account_registration_fee: Option<AssetItem>,
    //     account_data_size_range: Option<Range>,
    // ) -> TestResult<TransactionMetadata> {
    //     // programs
    //     let ProgramId {
    //         system_program,
    //         token_program,
    //         associated_token_program,
    //         registry: program_id,
    //         ..
    //     } = self.program_id;

    //     // signers
    //     let signers = &[sender.keypair()];
    //     let sender = sender.pubkey();

    //     // mint
    //     let revenue_mint = solana_pubkey::Pubkey::from(
    //         account_registration_fee
    //             .as_ref()
    //             .map(|x| x.asset)
    //             .unwrap_or(ACCOUNT_REGISTRATION_FEE_ASSET),
    //     );

    //     // pda
    //     let bump = self.pda.token_bump();
    //     let config = self.pda.token_config();
    //     let user_counter = self.pda.token_user_counter();
    //     let admin_rotation_state = self.pda.token_admin_rotation_state();

    //     // ata
    //     let revenue_app_ata = App::get_ata(&config, &revenue_mint);

    //     let accounts = types::init::TestAccounts {
    //         system_program,
    //         token_program,
    //         associated_token_program,
    //         sender,
    //         bump,
    //         config,
    //         user_counter,
    //         admin_rotation_state,
    //         revenue_mint,
    //         revenue_app_ata,
    //     }
    //     .to_account_metas();

    //     let mut instruction_data = types::init::InstructionData::default();
    //     if let Some(x) = rotation_timeout {
    //         instruction_data.set_rotation_timeout_flag(true);
    //         instruction_data.rotation_timeout.set(x);
    //     }
    //     if let Some(x) = account_registration_fee {
    //         instruction_data.set_account_registration_fee_flag(true);
    //         instruction_data.account_registration_fee = x;
    //     }
    //     if let Some(x) = account_data_size_range {
    //         instruction_data.set_account_data_size_range_flag(true);
    //         instruction_data.account_data_size_range = x;
    //     }

    //     let instruction_data = &instruction_data
    //         .serialize()
    //         .map_err(TestError::from_raw_error)?;

    //     send_tx_with_ix(
    //         self,
    //         &program_id,
    //         &accounts,
    //         &instruction_data,
    //         signers,
    //         &[],
    //     )
    // }

    // fn token_query_config(&self) -> TestResult<Config> {
    //     get_data(&self.litesvm, &self.pda.token_config())
    // }
}
