use {
    crate::helpers::suite::{
        core::{
            extension::{get_account_data, send_tx},
            App, ProgramId,
        },
        types::{
            addr_to_sol_pubkey, pin_pubkey_to_addr, to_optional_non_zero_pubkey, AppUser,
            SolPubkey, Target, TestError, TestResult,
        },
    },
    litesvm::types::TransactionMetadata,
    // pinocchio::pubkey::Pubkey,
    solana_address::Address,
    // solana_pubkey::Pubkey,
    spl_token_2022_interface::{
        extension::{
            metadata_pointer::MetadataPointer, BaseStateWithExtensions, StateWithExtensions,
        },
        state::Mint,
    },
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
use spl_token_metadata_interface::{
    instruction::{initialize, update_field},
    state::{Field, TokenMetadata},
};

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

        let mut ix = spl_token_metadata_interface::instruction::initialize(
            &token_2022_program_addr,
            metadata,
            update_authority,
            mint,
            mint_authority,
            name,
            symbol,
            uri,
        );

        ////DEBUG
        println!("transaction account at index 1 : {:?}", ix.accounts[1]);
        println!("update authority account : {:?}", update_authority);
        println!("metadata account : {:?}", metadata);
        // println!("ix accounts : {:?}", ix.accounts);
        //  println!("update_authority_balance : {:?}", app.get_coin_balance(update_authority));
        /////
        ///
        let acc = self
            .litesvm
            .get_account(&addr_to_sol_pubkey(update_authority))
            .unwrap();
        println!("update authority lamports in LiteSVM: {}", acc.lamports);

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

        println!("\n ix legacy accounts : {:?} \n", ix_legacy.accounts);

        if let Target::Proxy = target {
            ix_legacy.program_id = token_2022_proxy;
            ix_legacy.accounts.extend_from_slice(&additional_accounts);
        }

        ////@audit :: maybe send direct litesvm tx without catching error, if data is changed correctly go ahead then
        send_tx(
            &mut self.litesvm,
            &[ix_legacy],
            signers,
            self.is_log_displayed,
        )
    }

    // Add a query function
}
