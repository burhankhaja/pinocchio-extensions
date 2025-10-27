use {
    crate::helpers::{
        extensions::token_2022::{
            initialize_mint::Token2022InitializeMintExtension,
            initialize_multisig::Token2022InitializeMultisigExtension,
            metadata_pointer::Token2022MetadataPointerExtension,
            token_metadata::Token2022TokenMetadataExtension,
        },
        suite::{
            core::{extension, App},
            types::{pin_pubkey_to_addr, AppUser, PinPubkey, Target, TestResult},
        },
    }, pinocchio::pubkey::Pubkey, pretty_assertions::assert_eq, solana_keypair::Keypair, solana_signer::Signer, spl_pod::optional_keys::OptionalNonZeroPubkey, spl_token_2022_interface::extension::{metadata_pointer::MetadataPointer, ExtensionType}
};

use spl_token_metadata_interface::state::TokenMetadata;


#[test]
fn test_token_metadata() -> TestResult<()> {
    use solana_address::Address;

    let mut app = App::new(true);
    let (_, mint_keypair) = app.token_2022_try_create_mint_account(
        AppUser::Admin,
        None,
        Some(&[ExtensionType::MetadataPointer]),
    )?;

    let mint_pubkey = &mint_keypair.pubkey().to_bytes();
    let decimals: u8 = 6;
    let mint_authority = AppUser::Admin;
    let freeze_authority = Some(AppUser::Admin.pubkey());

    let metadata_pointer = MetadataPointer {
        authority: OptionalNonZeroPubkey::default(),
        metadata_address: OptionalNonZeroPubkey(pin_pubkey_to_addr(mint_pubkey)),
    };

    app.token_2022_try_initialize_metadata_pointer(
        Target::Spl,
        AppUser::Admin,
        mint_pubkey,
        None,
        Some(mint_pubkey),
    )?;

    // initialize mint before setting its metadata
    app.token_2022_try_initialize_mint(
        Target::Spl,
        AppUser::Admin,
        mint_pubkey,
        decimals,
        &mint_authority.pubkey(),
        freeze_authority.as_ref(),
    )?;

    ///// add token_metadata ix trait for app
    /// define it
    /// call it
    ///////////// @audit :: for debugging::: just build tx scaffold using conventional interface and check values ???
    /// lets check what data field is in token_metadat ix ::: and how is that unpacked ??? all that stuff
    use spl_token_metadata_interface::{
        instruction::{initialize as initialize_token_metadata, update_field},
        state::{Field, TokenMetadata},
    };

    let admin_pubkey = AppUser::Admin.pubkey();

    // Define Token metadata
    let token_metadata  = TokenMetadata {
        update_authority: OptionalNonZeroPubkey::try_from(Some(pin_pubkey_to_addr(&admin_pubkey))).unwrap(),
        mint: pin_pubkey_to_addr(mint_pubkey),
        name: "OPOS".to_string(),
        symbol : "OPS".to_string(),
        uri : "https://raw.githubusercontent.com/solana-developers/opos-asset/main/assets/DeveloperPortal/metadata.json".to_string(),
        additional_metadata: vec![("description".to_string(),"only possible on Solana".to_string())]
    };

    let mint_raw_data_before = extension::get_account_data(&app, &mint_pubkey)?;

    println!(
        "admin pubkey: {:?}",
        solana_pubkey::Pubkey::from(admin_pubkey)
    );
    println!(
        "admin balance before : {:?}",
        app.get_coin_balance(&solana_pubkey::Pubkey::from(admin_pubkey))
    );

    let alice_pubkey = AppUser::Alice.pubkey();

    // dev : fund metadata account a bit more to avoid : litesvm rent errors: Error: TestError { info: "Not a ProgramError and custom_program_error_idx isn't found", index: None }
    /*
         FailedTransactionMetadata {
         err: InsufficientFundsForRent {
             account_index: 1,
         },
     }
    */
    app.litesvm
        .airdrop(&solana_pubkey::Pubkey::from(*mint_pubkey), 2_000_000);

    app.token_2022_try_initialize_token_metadata(
        Target::Spl,
        AppUser::Admin,
        &pin_pubkey_to_addr(&mint_pubkey),
        &pin_pubkey_to_addr(&admin_pubkey),
        &pin_pubkey_to_addr(&mint_pubkey),
        &pin_pubkey_to_addr(&admin_pubkey),
        token_metadata.name.to_string(),
        token_metadata.symbol.to_string(),
        token_metadata.uri.to_string(),
    )?;

    // // Create update field instructions from token_metadata.additional_metadata
    // // Additional metadata must be initialized separately using the update_field instruction
    // // If the field already exists, it will be updated instead of creating a new field
    // let update_field_instructions: Vec<_> = token_metadata
    //     .additional_metadata
    //     .iter()
    //     .map(|(key, value)| {
    //         update_field(
    //             &TOKEN_2022_PROGRAM_ID,
    //             &mint.pubkey(),
    //             &fee_payer.pubkey(),
    //             Field::Key(key.clone()),
    //             value.clone(),
    //         )
    //     })
    //     .collect();

    // use crate::helpers::suite::core::get_account_data
    let mint_raw_data_after = extension::get_account_data(&app, &mint_pubkey)?;

    //// prints
    println!("BEFORE >>>>>>>>>><<<<<<<<<< : {:?}", mint_raw_data_before);
    println!("AFTER  >>>>>>>>>><<<<<<<<<< : {:?}", mint_raw_data_after);

    Ok(())
}

#[test]
fn proxy_test_token_metadata() -> TestResult<()> {
    use solana_address::Address;

    let mut app = App::new(true);
    let (_, mint_keypair) = app.token_2022_try_create_mint_account(
        AppUser::Admin,
        None,
        Some(&[ExtensionType::MetadataPointer]),
    )?;

    let mint_pubkey = &mint_keypair.pubkey().to_bytes();
    let decimals: u8 = 6;
    let mint_authority = AppUser::Admin;
    let freeze_authority = Some(AppUser::Admin.pubkey());

    let metadata_pointer = MetadataPointer {
        authority: OptionalNonZeroPubkey::default(),
        metadata_address: OptionalNonZeroPubkey(pin_pubkey_to_addr(mint_pubkey)),
    };

    app.token_2022_try_initialize_metadata_pointer(
        Target::Spl,
        AppUser::Admin,
        mint_pubkey,
        None,
        Some(mint_pubkey),
    )?;

    // initialize mint before setting its metadata
    app.token_2022_try_initialize_mint(
        Target::Spl,
        AppUser::Admin,
        mint_pubkey,
        decimals,
        &mint_authority.pubkey(),
        freeze_authority.as_ref(),
    )?;


    let admin_pubkey = AppUser::Admin.pubkey();

    // Define Token metadata
    let token_metadata  = TokenMetadata {
        update_authority: OptionalNonZeroPubkey::try_from(Some(pin_pubkey_to_addr(&admin_pubkey))).unwrap(),
        mint: pin_pubkey_to_addr(mint_pubkey),
        name: "OPOS".to_string(),
        symbol : "OPS".to_string(),
        uri : "https://raw.githubusercontent.com/solana-developers/opos-asset/main/assets/DeveloperPortal/metadata.json".to_string(),
        additional_metadata: vec![("description".to_string(),"only possible on Solana".to_string())]
    };


    let mint_raw_data_before = extension::get_account_data(&app, &mint_pubkey)?;




    // note : fund `metadata` bit more to avoid litesvm rent errors
    app.litesvm
        .airdrop(&solana_pubkey::Pubkey::from(*mint_pubkey), 2_000_000);

    app.token_2022_try_initialize_token_metadata(
        Target::Proxy,
        AppUser::Admin,
        &pin_pubkey_to_addr(&mint_pubkey),
        &pin_pubkey_to_addr(&admin_pubkey),
        &pin_pubkey_to_addr(&mint_pubkey),
        &pin_pubkey_to_addr(&admin_pubkey),
        token_metadata.name.to_string(),
        token_metadata.symbol.to_string(),
        token_metadata.uri.to_string(),
    )?;


    Ok(())
}

#[test]
fn decode_data() {
    let token_metadata = [1, 0, 0, 0, 219, 13, 46, 235, 155, 0, 103, 86, 67, 139, 99, 108, 203, 151, 240, 193, 9, 36, 6, 10, 91, 89, 180, 123, 28, 168, 121, 38, 63, 127, 68, 123, 0, 0, 0, 0, 0, 0, 0, 0, 6, 1, 1, 0, 0, 0, 219, 13, 46, 235, 155, 0, 103, 86, 67, 139, 99, 108, 203, 151, 240, 193, 9, 36, 6, 10, 91, 89, 180, 123, 28, 168, 121, 38, 63, 127, 68, 123, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 18, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 96, 188, 192, 176, 62, 209, 111, 88, 22, 76, 163, 199, 172, 235, 9, 22, 8, 255, 8, 38, 167, 190, 14, 9, 91, 96, 73, 221, 165, 35, 172, 169, 19, 0, 191, 0, 219, 13, 46, 235, 155, 0, 103, 86, 67, 139, 99, 108, 203, 151, 240, 193, 9, 36, 6, 10, 91, 89, 180, 123, 28, 168, 121, 38, 63, 127, 68, 123, 96, 188, 192, 176, 62, 209, 111, 88, 22, 76, 163, 199, 172, 235, 9, 22, 8, 255, 8, 38, 167, 190, 14, 9, 91, 96, 73, 221, 165, 35, 172, 169, 4, 0, 0, 0, 79, 80, 79, 83, 3, 0, 0, 0, 79, 80, 83, 104, 0, 0, 0, 104, 116, 116, 112, 115, 58, 47, 47, 114, 97, 119, 46, 103, 105, 116, 104, 117, 98, 117, 115, 101, 114, 99, 111, 110, 116, 101, 110, 116, 46, 99, 111, 109, 47, 115, 111, 108, 97, 110, 97, 45, 100, 101, 118, 101, 108, 111, 112, 101, 114, 115, 47, 111, 112, 111, 115, 45, 97, 115, 115, 101, 116, 47, 109, 97, 105, 110, 47, 97, 115, 115, 101, 116, 115, 47, 68, 101, 118, 101, 108, 111, 112, 101, 114, 80, 111, 114, 116, 97, 108, 47, 109, 101, 116, 97, 100, 97, 116, 97, 46, 106, 115, 111, 110, 0, 0, 0, 0];
 
    println!("data from 165 index: {:?}", &token_metadata[165..165+8]); //[1, 18, 0, 64, 0, 0, 0, 0] // could be initialize
}



// debug____discriminators___directly
#[ignore]
#[test]
fn test_discriminators() {

    use solana_address::Address;

    let mut app = App::new(true);
    let mint_keypair = Keypair::new();
    let mint_pubkey = &mint_keypair.pubkey().to_bytes();
    // let decimals: u8 = 6;
    // let mint_authority = AppUser::Admin;
    // let freeze_authority = Some(AppUser::Admin.pubkey());

    let admin_pubkey = AppUser::Admin.pubkey();

    // Define Token metadata
    let token_metadata  = TokenMetadata {
        update_authority: OptionalNonZeroPubkey::try_from(Some(pin_pubkey_to_addr(&admin_pubkey))).unwrap(),
        mint: pin_pubkey_to_addr(mint_pubkey),
        name: "OPOS".to_string(),
        symbol : "OPS".to_string(),
        uri : "https://raw.githubusercontent.com/solana-developers/opos-asset/main/assets/DeveloperPortal/metadata.json".to_string(),
        additional_metadata: vec![("description".to_string(),"only possible on Solana".to_string())]
    };

    // app.token_2022_try_initialize_token_metadata(
    //     Target::Proxy,
    //     AppUser::Admin,
    //     &pin_pubkey_to_addr(&mint_pubkey),
    //     &pin_pubkey_to_addr(&admin_pubkey),
    //     &pin_pubkey_to_addr(&mint_pubkey),
    //     &pin_pubkey_to_addr(&admin_pubkey),
    //     token_metadata.name.to_string(),
    //     token_metadata.symbol.to_string(),
    //     token_metadata.uri.to_string(),
    // )?;

    use crate::helpers::suite::core::ProgramId;
       let ProgramId {
            token_2022_program,
            token_2022_proxy,
            ..
        } = app.program_id;

        let token_2022_program_addr = Address::new_from_array(token_2022_program.to_bytes());

        // let signers = &[sender.keypair()];
        let signers = &[AppUser::Admin.keypair()];

        let ix = spl_token_metadata_interface::instruction::initialize(
            &token_2022_program_addr,
            &pin_pubkey_to_addr(&mint_pubkey),
            &pin_pubkey_to_addr(&admin_pubkey),
            &pin_pubkey_to_addr(&mint_pubkey),
            &pin_pubkey_to_addr(&admin_pubkey),
        token_metadata.name.to_string(),
        token_metadata.symbol.to_string(),
        token_metadata.uri.to_string(),
        );

     let ix_data = ix.data;
     println!("ix_data : {:?}", ix_data);

     /* Log: 
     ix_data : [210, 225, 30, 162, 88, 184, 77, 141, 4, 0, 0, 0, 79, 80, 79, 83, 3, 0, 0, 0, 79, 80, 83, 104, 0, 0, 0, 104, 116, 116, 112, 115, 58, 47, 47, 114, 97, 119, 46, 103, 105, 116, 104, 117, 98, 117, 115, 101, 114, 99, 111, 110, 116, 101, 110, 116, 46, 99, 111, 109, 47, 115, 111, 108, 97, 110, 97, 45, 100, 101, 118, 101, 108, 111, 112, 101, 114, 115, 47, 111, 112, 111, 115, 45, 97, 115, 115, 101, 116, 47, 109, 97, 105, 110, 47, 97, 115, 115, 101, 116, 115, 47, 68, 101, 118, 101, 108, 111, 112, 101, 114, 80, 111, 114, 116, 97, 108, 47, 109, 101, 116, 97, 100, 97, 116, 97, 46, 106, 115, 111, 110] */

     // may_be_discriminator ::  [210, 225, 30, 162, 88, 184, 77, 141]


     // Unpack to validate
     use spl_token_metadata_interface::instruction::TokenMetadataInstruction;
     use spl_token_metadata_interface::instruction::Initialize;

     let unpacked_data = TokenMetadataInstruction::unpack(&ix_data).unwrap();

     println!("unpacked_data: {:?}", unpacked_data);

     /*  Log : 
     unpacked_data: Initialize(Initialize { name: "OPOS", symbol: "OPS", uri: "https://raw.githubusercontent.com/solana-developers/opos-asset/main/assets/DeveloperPortal/metadata.json" })
     */
    
    print!("upto ..8: {:?}", &ix_data[..8]);

     let initialize_discriminator = spl_discriminator::ArrayDiscriminator::try_from(&ix_data[..8]).unwrap();
     println!("Get decoded discriminator from bytes ::: {:?}", initialize_discriminator);

 use spl_discriminator::{discriminator::ArrayDiscriminator, SplDiscriminate};

    //  use spl_token_metadata_interface::SplDiscriminate;
     println!("\n real initialize discriminator : {:?} \n", Initialize::SPL_DISCRIMINATOR_SLICE);
}



// dev :: Important helper test
#[test]
fn get_token_metadata_original_discriminators() {
 use spl_discriminator::{discriminator::ArrayDiscriminator, SplDiscriminate};
 use spl_token_metadata_interface::instruction::{Emit, Initialize, RemoveKey, TokenMetadataInstruction, UpdateAuthority, UpdateField};

 println!("\n Initialize discriminator : {:?} \n", Initialize::SPL_DISCRIMINATOR_SLICE);
 println!("\n UpdateField discriminator : {:?} \n", UpdateField::SPL_DISCRIMINATOR_SLICE);
 println!("\n UpdateAuthority discriminator : {:?} \n", UpdateAuthority::SPL_DISCRIMINATOR_SLICE);
 println!("\n RemoveKey discriminator : {:?} \n", RemoveKey::SPL_DISCRIMINATOR_SLICE);
 println!("\n Emit discriminator : {:?} \n", Emit::SPL_DISCRIMINATOR_SLICE);

 /*
 // Logs :: 
  Initialize discriminator : [210, 225, 30, 162, 88, 184, 77, 141] 
  UpdateField discriminator : [221, 233, 49, 45, 181, 202, 220, 200] 
  UpdateAuthority discriminator : [215, 228, 166, 228, 84, 100, 86, 123] 
  RemoveKey discriminator : [234, 18, 32, 56, 89, 141, 37, 181] 
  Emit discriminator : [250, 166, 180, 250, 13, 12, 184, 70] 
 
  */
 
}

//@note :: Check how `token_group's` DISCRIMINATOR  is routed via Proxy ; notice that ix_discriminators are encoded in u64 from 8 byte discriminators, you will have to do the same for token_metadata ??