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
    },
    pinocchio::pubkey::Pubkey,
    pretty_assertions::assert_eq,
    solana_signer::Signer,
    spl_pod::optional_keys::OptionalNonZeroPubkey,
    spl_token_2022_interface::extension::{metadata_pointer::MetadataPointer, ExtensionType},
};
// use crate::helpers::suite::core::get_account_data
/*
  // Generate keypair for the mint
    let mint = Keypair::new();

    // Define Token metadata
    let token_metadata  = TokenMetadata {
        update_authority: Some(fee_payer.pubkey()).try_into()?,
        mint: mint.pubkey(),
        name: "OPOS".to_string(),
        symbol : "OPS".to_string(),
        uri : "https://raw.githubusercontent.com/solana-developers/opos-asset/main/assets/DeveloperPortal/metadata.json".to_string(),
        additional_metadata: vec![("description".to_string(),"only possible on Solana".to_string())]
    };

    // Calculate space for mint with metadata pointer and token metadata extensions
    let mint_space =
        ExtensionType::try_calculate_account_len::<Mint>(&[ExtensionType::MetadataPointer])?;

    let metadata_len = token_metadata.tlv_size_of()?;

    let mint_rent = client
        .get_minimum_balance_for_rent_exemption(mint_space + metadata_len)
        .await?;

    // Instruction to create new account for mint (token22)
    let create_mint_account_instruction = create_account(
        &fee_payer.pubkey(),    // payer
        &mint.pubkey(),         // new account (mint)
        mint_rent,              // lamports
        mint_space as u64,      // space
        &TOKEN_2022_PROGRAM_ID, // program id
    );

    // Instruction to initialize metadata pointer (pointing to itself for self-managed metadata)
    let initialize_metadata_pointer_instruction = initialize_metadata_pointer(
        &TOKEN_2022_PROGRAM_ID,
        &mint.pubkey(),
        Some(fee_payer.pubkey()), // authority
        Some(mint.pubkey()),      // metadata address (pointing to self)
    )?;

    // Instruction to initialize mint account data
    let initialize_mint_instruction = initialize_mint(
        &TOKEN_2022_PROGRAM_ID,    // program id
        &mint.pubkey(),            // mint
        &fee_payer.pubkey(),       // mint authority
        Some(&fee_payer.pubkey()), // freeze authority
        9,                         // decimals
    )?;

    // Instruction to initialize token metadata
    let initialize_metadata_instruction = initialize_token_metadata(
        &TOKEN_2022_PROGRAM_ID,            // program id
        &mint.pubkey(),                    //metadata
        &fee_payer.pubkey(),               // update authority
        &mint.pubkey(),                    // mint
        &fee_payer.pubkey(),               // mint authority
        token_metadata.name.to_string(),   // name
        token_metadata.symbol.to_string(), // symbol
        token_metadata.uri.to_string(),    // uri
    );
*/

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

    // Instruction to initialize token metadata
    // let initialize_metadata_instruction = initialize_token_metadata(
    //     &TOKEN_2022_PROGRAM_ID,            // program id
    //     &mint_pubkey(),                    //metadata
    //     &fee_payer.pubkey(),               // update authority
    //     &mint_pubkey,                    // mint
    //     &admin_pubkey,               // mint authority
    //     token_metadata.name.to_string(),   // name
    //     token_metadata.symbol.to_string(), // symbol
    //     token_metadata.uri.to_string(),    // uri
    // );
    // app.token_2022_try_initialize_token_metadata()

    /*(&mut self, target: Target, sender: AppUser, metadata: &Address, update_authority: &Address, mint: &Address, mint_authority: &Address, name: String, symbol: String, uri: String) */

    /**   pubkey --> addr
     *     let token_2022_program_addr = Address::new_from_array(token_2022_program.to_bytes());
     */
    let mut mint_raw_data_before = extension::get_account_data(&app, &mint_pubkey)?;

    println!(
        "admin pubkey: {:?}",
        solana_pubkey::Pubkey::from(admin_pubkey)
    );
    println!(
        "admin balance before : {:?}",
        app.get_coin_balance(&solana_pubkey::Pubkey::from(admin_pubkey))
    );

    let alice_pubkey = AppUser::Alice.pubkey();

    // fund metadata account a bit more to avoid : litesvm rent errors:
    //     FailedTransactionMetadata {
    //     err: InsufficientFundsForRent {
    //         account_index: 1,
    //     },
    // }
    // Error: TestError { info: "Not a ProgramError and custom_program_error_idx isn't found", index: None }
    app.litesvm
        .airdrop(&solana_pubkey::Pubkey::from(*mint_pubkey), 2_000_000); // SCOOBY DOOBY FIX

    app.token_2022_try_initialize_token_metadata(
        Target::Spl,
        AppUser::Admin,
        &pin_pubkey_to_addr(&mint_pubkey),
        &pin_pubkey_to_addr(&admin_pubkey /*alice_pubkey*/),
        &pin_pubkey_to_addr(&mint_pubkey),
        &pin_pubkey_to_addr(&admin_pubkey),
        token_metadata.name.to_string(),
        token_metadata.symbol.to_string(),
        token_metadata.uri.to_string(),
    )?; //@audit :: may be try tx builder one and send transaction directly after>>>>

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
    let mut mint_raw_data_after = extension::get_account_data(&app, &mint_pubkey)?;

    //// prints
    println!("BEFORE >>>>>>>>>><<<<<<<<<< : {:?}", mint_raw_data_before);
    println!("AFTER  >>>>>>>>>><<<<<<<<<< : {:?}", mint_raw_data_after);

    Ok(())
}

// #[test]
// fn initialize_metadata_pointer_with_default_authority() -> TestResult<()> {
//     let mut app = App::new(false);
//     let (_, mint_keypair) = app.token_2022_try_create_mint_account(
//         AppUser::Admin,
//         None,
//         Some(&[ExtensionType::MetadataPointer]),
//     )?;

//     let mint_pubkey = &mint_keypair.pubkey().to_bytes();
//     let decimals: u8 = 6;
//     let mint_authority = AppUser::Admin;
//     let freeze_authority = Some(AppUser::Admin.pubkey());

//     let metadata_pointer = MetadataPointer {
//         authority: OptionalNonZeroPubkey::default(),
//         metadata_address: OptionalNonZeroPubkey(pin_pubkey_to_addr(mint_pubkey)),
//     };

//     app.token_2022_try_initialize_metadata_pointer(
//         Target::Spl,
//         AppUser::Admin,
//         mint_pubkey,
//         None,
//         Some(mint_pubkey),
//     )?;

//     app.token_2022_try_initialize_mint(
//         Target::Spl,
//         AppUser::Admin,
//         mint_pubkey,
//         decimals,
//         &mint_authority.pubkey(),
//         freeze_authority.as_ref(),
//     )?;

//     assert_eq!(
//         app.token_2022_query_metadata_pointer(Target::Spl, mint_pubkey)?,
//         metadata_pointer
//     );

//     Ok(())
// }

/////////////////////////////////////////////////////////////////////////
///
///
///

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

    // Instruction to initialize token metadata
    // let initialize_metadata_instruction = initialize_token_metadata(
    //     &TOKEN_2022_PROGRAM_ID,            // program id
    //     &mint_pubkey(),                    //metadata
    //     &fee_payer.pubkey(),               // update authority
    //     &mint_pubkey,                    // mint
    //     &admin_pubkey,               // mint authority
    //     token_metadata.name.to_string(),   // name
    //     token_metadata.symbol.to_string(), // symbol
    //     token_metadata.uri.to_string(),    // uri
    // );
    // app.token_2022_try_initialize_token_metadata()

    /*(&mut self, target: Target, sender: AppUser, metadata: &Address, update_authority: &Address, mint: &Address, mint_authority: &Address, name: String, symbol: String, uri: String) */

    /**   pubkey --> addr
     *     let token_2022_program_addr = Address::new_from_array(token_2022_program.to_bytes());
     */
    let mut mint_raw_data_before = extension::get_account_data(&app, &mint_pubkey)?;

    println!(
        "admin pubkey: {:?}",
        solana_pubkey::Pubkey::from(admin_pubkey)
    );
    println!(
        "admin balance before : {:?}",
        app.get_coin_balance(&solana_pubkey::Pubkey::from(admin_pubkey))
    );

    let alice_pubkey = AppUser::Alice.pubkey();

    // fund metadata account a bit more to avoid : litesvm rent errors:
    //     FailedTransactionMetadata {
    //     err: InsufficientFundsForRent {
    //         account_index: 1,
    //     },
    // }
    // Error: TestError { info: "Not a ProgramError and custom_program_error_idx isn't found", index: None }
    app.litesvm
        .airdrop(&solana_pubkey::Pubkey::from(*mint_pubkey), 2_000_000); // SCOOBY DOOBY FIX

    app.token_2022_try_initialize_token_metadata(
        Target::Proxy,
        AppUser::Admin,
        &pin_pubkey_to_addr(&mint_pubkey),
        &pin_pubkey_to_addr(&admin_pubkey /*alice_pubkey*/),
        &pin_pubkey_to_addr(&mint_pubkey),
        &pin_pubkey_to_addr(&admin_pubkey),
        token_metadata.name.to_string(),
        token_metadata.symbol.to_string(),
        token_metadata.uri.to_string(),
    )?; //@audit :: may be try tx builder one and send transaction directly after>>>>

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
    let mut mint_raw_data_after = extension::get_account_data(&app, &mint_pubkey)?;

    //// prints
    println!("BEFORE >>>>>>>>>><<<<<<<<<< : {:?}", mint_raw_data_before);
    println!("AFTER  >>>>>>>>>><<<<<<<<<< : {:?}", mint_raw_data_after);

    Ok(())
}
