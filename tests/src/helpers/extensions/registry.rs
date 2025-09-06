use {
    crate::helpers::suite::{
        core::{
            extension::{get_data, send_tx},
            get_test_error_from_logs, App, ProgramId,
        },
        types::{
            addr_to_sol_pubkey, pin_pubkey_to_addr, pin_to_sol_pubkey, AppUser, SolPubkey,
            TestError, TestResult,
        },
    },
    litesvm::types::TransactionMetadata,
    pinocchio::pubkey::Pubkey,
    solana_keypair::Keypair,
    solana_signer::Signer,
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
        println!("ix_legacy: {:?}\n", &ix_legacy);

        // to avoid AlreadyProcessed error
        self.litesvm.expire_blockhash();

        let mint_authority = sender.keypair();
        let message = solana_message::Message::new(&[ix_legacy], Some(&mint_authority.pubkey()));
        let mut transaction = solana_transaction::Transaction::new_unsigned(message);
        let blockhash = self.litesvm.latest_blockhash();
        transaction.sign(&[mint_authority], blockhash);

        println!("transaction: {:?}\n", &transaction);

        match self.litesvm.send_transaction(transaction) {
            Ok(x) => {
                let logs = &x.logs;

                if self.is_log_displayed {
                    println!("Transaction logs: {:#?}\n", logs);
                }

                Ok(x)
            }
            Err(e) => {
                let logs = &e.meta.logs;

                if self.is_log_displayed {
                    println!("Transaction logs: {:#?}\n", logs);
                }

                Err(get_test_error_from_logs(logs))
            }
        }
    }

    // fn token_query_config(&self) -> TestResult<Config> {
    //     get_data(&self.litesvm, &self.pda.token_config())
    // }
}
