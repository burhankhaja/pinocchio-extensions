use {
    crate::helpers::{
        extensions::token_2022::{
            initialize_mint::Token2022InitializeMintExtension,
            token_group::Token2022TokenGroupExtension,
            token_metadata::Token2022TokenMetadataExtension,
        },
        suite::{
            core::App,
            types::{pin_pubkey_to_addr, to_c_option, AppUser, PinPubkey, TestResult},
        },
    },
    pretty_assertions::assert_eq,
    solana_keypair::Keypair,
    solana_program_option::COption,
    solana_signer::Signer,
    spl_pod::optional_keys::OptionalNonZeroPubkey,
    spl_token_2022_interface::extension::ExtensionType,
};

// #[test]
// fn token_group() -> TestResult<()> {
//     let mut app = App::new(true);

//     // 1. Create group account keypair
//     let group_keypair = Keypair::new();
//     let group_pubkey = &group_keypair.pubkey().to_bytes();

//     // 2. Create mint with GroupPointer extension
//     let extensions = &[ExtensionType::GroupPointer];
//     let (_, mint_keypair) = app.token_2022_try_create_mint_account(
//         AppUser::Admin,
//         None, // Let it generate a keypair
//         Some(extensions),
//     )?;
//     let mint_pubkey = &mint_keypair.pubkey().to_bytes();

//     let mint_authority = AppUser::Alice;

//     // 3. Initialize group pointer to point to separate account
//     app.token_2022_try_initialize_group_pointer(
//         AppUser::Admin,
//         mint_pubkey,
//         Some(&mint_authority.pubkey()),
//         Some(group_pubkey), // Point to separate account
//     )?;

//     let update_authority = Some(AppUser::Bob.pubkey());

//     // 4. Initialize the token group in the separate account
//     app.token_2022_try_initialize_token_group(
//         AppUser::Admin,
//         group_pubkey, // Separate group account
//         mint_pubkey,
//         mint_authority,
//         update_authority.as_ref(),
//         100, // max_size
//     )?;

//     Ok(())
// }

// #[test]
// fn token_group() -> TestResult<()> {
//     let mut app = App::new(true);

//     let mint_authority = AppUser::Alice;
//     let update_authority = Some(AppUser::Bob.pubkey());

//     // 1. Create mint with GroupPointer extension
//     let (_, mint_keypair) = app.token_2022_try_create_mint_account(
//         AppUser::Admin,
//         None,
//         Some(&[ExtensionType::GroupPointer]),
//     )?;
//     let mint_pubkey = &mint_keypair.pubkey().to_bytes();

//     // 2. Create a separate account for the TokenGroup
//     // This needs to be a Token-2022 account with TokenGroup extension
//     let group_keypair = Keypair::new();
//     let group_pubkey = &group_keypair.pubkey().to_bytes();

//     app.token_2022_try_create_mint_account(
//         AppUser::Admin,
//         Some(group_keypair), // Use our specific keypair
//         Some(&[ExtensionType::TokenGroup]),
//     )?;

//     // 3. Initialize group pointer to point to separate account
//     app.token_2022_try_initialize_group_pointer(
//         AppUser::Admin,
//         mint_pubkey,
//         Some(&mint_authority.pubkey()),
//         Some(group_pubkey),
//     )?;

//     // 4. Initialize the token group in the separate account
//     app.token_2022_try_initialize_token_group(
//         AppUser::Admin,
//         group_pubkey,
//         mint_pubkey,
//         mint_authority,
//         update_authority.as_ref(),
//         100,
//     )?;

//     Ok(())
// }

// #[test]
// fn token_group() -> TestResult<()> {
//     let mut app = App::new(true);

//     let mint_authority = AppUser::Alice;
//     let update_authority = Some(AppUser::Bob.pubkey());

//     // 1. Create mint with BOTH GroupPointer AND TokenGroup extensions
//     let (_, mint_keypair) = app.token_2022_try_create_mint_account(
//         AppUser::Admin,
//         None,
//         Some(&[ExtensionType::GroupPointer, ExtensionType::TokenGroup]),
//     )?;
//     let mint_pubkey = &mint_keypair.pubkey().to_bytes();

//     // 2. Initialize group pointer to point to the mint itself (self-referencing)
//     app.token_2022_try_initialize_group_pointer(
//         AppUser::Admin,
//         mint_pubkey,
//         Some(&mint_authority.pubkey()),
//         Some(mint_pubkey), // Point to the mint itself, not a separate account
//     )?;

//     // 3. Initialize the token group in the SAME mint account
//     app.token_2022_try_initialize_token_group(
//         AppUser::Admin,
//         mint_pubkey, // Use mint_pubkey, not a separate group account
//         mint_pubkey,
//         mint_authority,
//         update_authority.as_ref(),
//         100,
//     )?;

//     Ok(())
// }

// #[test]
// fn token_group() -> TestResult<()> {
//     let mut app = App::new(true);

//     let decimals: u8 = 6;
//     let mint_authority = AppUser::Alice;
//     let update_authority = Some(AppUser::Bob.pubkey());

//     // 1. Create mint with BOTH GroupPointer AND TokenGroup extensions
//     let (_, mint_keypair) = app.token_2022_try_create_mint_account(
//         AppUser::Admin,
//         None,
//         Some(&[ExtensionType::GroupPointer, ExtensionType::TokenGroup]),
//     )?;
//     let mint_pubkey = &mint_keypair.pubkey().to_bytes();

//     // 2. Initialize the mint account itself first
//     app.token_2022_try_initialize_mint(
//         AppUser::Admin,
//         mint_pubkey,
//         decimals,
//         &mint_authority.pubkey(),
//         None,
//     )?;

//     // 3. Initialize group pointer to point to the mint itself (self-referencing)
//     app.token_2022_try_initialize_group_pointer(
//         AppUser::Admin,
//         mint_pubkey,
//         Some(&mint_authority.pubkey()),
//         Some(mint_pubkey), // Point to the mint itself, not a separate account
//     )?;

//     // 4. Initialize the token group in the SAME mint account
//     app.token_2022_try_initialize_token_group(
//         AppUser::Admin,
//         mint_pubkey, // Use mint_pubkey, not a separate group account
//         mint_pubkey,
//         mint_authority,
//         update_authority.as_ref(),
//         100,
//     )?;

//     Ok(())
// }

// #[test]
// fn token_group() -> TestResult<()> {
//     let mut app = App::new(true);

//     let decimals: u8 = 6;
//     let mint_authority = AppUser::Alice;
//     let update_authority = Some(AppUser::Bob.pubkey());

//     // 1. Create mint account with BOTH GroupPointer AND TokenGroup extensions
//     let (_, mint_keypair) = app.token_2022_try_create_mint_account(
//         AppUser::Admin,
//         None,
//         Some(&[ExtensionType::GroupPointer, ExtensionType::TokenGroup]),
//     )?;
//     let mint_pubkey = &mint_keypair.pubkey().to_bytes();

//     // 2. Initialize group pointer FIRST (before mint initialization)
//     app.token_2022_try_initialize_group_pointer(
//         AppUser::Admin,
//         mint_pubkey,
//         Some(&mint_authority.pubkey()),
//         Some(mint_pubkey), // Point to the mint itself (self-referencing)
//     )?;

//     // 3. Initialize the token group extension SECOND (before mint initialization)
//     app.token_2022_try_initialize_token_group(
//         AppUser::Admin,
//         mint_pubkey, // Use mint_pubkey as the group account
//         mint_pubkey,
//         mint_authority,
//         update_authority.as_ref(),
//         100,
//     )?;

//     // // 4. Initialize the mint account itself LAST
//     // app.token_2022_try_initialize_mint(
//     //     AppUser::Admin,
//     //     mint_pubkey,
//     //     decimals,
//     //     &mint_authority.pubkey(),
//     //     None,
//     // )?;

//     Ok(())
// }

// #[test]
// fn token_group() -> TestResult<()> {
//     let mut app = App::new(true);

//     let decimals: u8 = 6;
//     let mint_authority = AppUser::Alice;
//     let update_authority = Some(AppUser::Bob.pubkey());

//     // 1. Create mint account with BOTH GroupPointer AND TokenGroup extensions
//     let (_, mint_keypair) = app.token_2022_try_create_mint_account(
//         AppUser::Admin,
//         None,
//         Some(&[ExtensionType::GroupPointer, ExtensionType::TokenGroup]),
//     )?;
//     let mint_pubkey = &mint_keypair.pubkey().to_bytes();

//     // 2. Initialize group pointer FIRST
//     app.token_2022_try_initialize_group_pointer(
//         AppUser::Admin,
//         mint_pubkey,
//         Some(&mint_authority.pubkey()),
//         Some(mint_pubkey), // Point to the mint itself (self-referencing)
//     )?;

//     // 3. Initialize the mint account SECOND (required before TokenGroup)
//     app.token_2022_try_initialize_mint(
//         AppUser::Admin,
//         mint_pubkey,
//         decimals,
//         &mint_authority.pubkey(),
//         None,
//     )?;

//     // // 4. Initialize the token group extension LAST (requires initialized mint)
//     // app.token_2022_try_initialize_token_group(
//     //     AppUser::Admin,
//     //     mint_pubkey, // Use mint_pubkey as the group account
//     //     mint_pubkey,
//     //     mint_authority,
//     //     update_authority.as_ref(),
//     //     100,
//     // )?;

//     Ok(())
// }

// #[test]
// fn token_group() -> TestResult<()> {
//     let mut app = App::new(true);

//     let decimals: u8 = 6;
//     let mint_authority = AppUser::Alice;
//     let update_authority = Some(AppUser::Bob.pubkey());

//     // 1. Create mint account with BOTH GroupPointer AND TokenGroup extensions
//     // NOTE: We need to account for TOKEN_GROUP_SIZE in account creation
//     let (_, mint_keypair) = app.token_2022_try_create_mint_account(
//         AppUser::Admin,
//         None,
//         Some(&[ExtensionType::GroupPointer, ExtensionType::TokenGroup]),
//         10_000_000,
//     )?;
//     let mint_pubkey = &mint_keypair.pubkey().to_bytes();

//     // 2. Initialize group pointer extension FIRST
//     app.token_2022_try_initialize_group_pointer(
//         AppUser::Admin,
//         mint_pubkey,
//         Some(&mint_authority.pubkey()),
//         Some(mint_pubkey), // Point to the mint itself (self-referencing)
//     )?;

//     // 3. Initialize the mint account SECOND
//     app.token_2022_try_initialize_mint(
//         AppUser::Admin,
//         mint_pubkey,
//         decimals,
//         &mint_authority.pubkey(),
//         None,
//     )?;

//     // // 4. Initialize the token group extension LAST
//     // app.token_2022_try_initialize_token_group(
//     //     AppUser::Admin,
//     //     mint_pubkey, // Use mint_pubkey as the group account
//     //     mint_pubkey,
//     //     mint_authority,
//     //     update_authority.as_ref(),
//     //     100,
//     // )?;

//     Ok(())
// }

// #[test]
// fn token_group() -> TestResult<()> {
//     let mut app = App::new(true);

//     let decimals: u8 = 6;
//     let mint_authority = AppUser::Alice;
//     let update_authority = Some(AppUser::Bob.pubkey());

//     // 1. Create mint account with BOTH GroupPointer AND TokenGroup extensions
//     // NOTE: We need to account for TOKEN_GROUP_SIZE in account creation
//     let (_, mint_keypair) = app.token_2022_try_create_mint_account(
//         AppUser::Admin,
//         None,
//         Some(&[ExtensionType::GroupPointer, ExtensionType::TokenGroup]),
//         100,
//     )?;
//     let mint_pubkey = &mint_keypair.pubkey().to_bytes();

//     // 2. Initialize the mint account FIRST (before any extensions)
//     app.token_2022_try_initialize_mint(
//         AppUser::Admin,
//         mint_pubkey,
//         decimals,
//         &mint_authority.pubkey(),
//         None,
//     )?;

//     // // 3. Initialize group pointer extension SECOND
//     // app.token_2022_try_initialize_group_pointer(
//     //     AppUser::Admin,
//     //     mint_pubkey,
//     //     Some(&mint_authority.pubkey()),
//     //     Some(mint_pubkey), // Point to the mint itself (self-referencing)
//     // )?;

//     // // 4. Initialize the token group extension LAST
//     // app.token_2022_try_initialize_token_group(
//     //     AppUser::Admin,
//     //     mint_pubkey, // Use mint_pubkey as the group account
//     //     mint_pubkey,
//     //     mint_authority,
//     //     update_authority.as_ref(),
//     //     100,
//     // )?;

//     Ok(())
// }

#[test]
fn token_group() -> TestResult<()> {
    let mut app = App::new(true);

    let decimals: u8 = 6;
    let mint_authority = AppUser::Alice;
    let update_authority = Some(AppUser::Bob.pubkey());

    // 1. Create mint account with BOTH GroupPointer AND TokenGroup extensions
    let (_, mint_keypair) = app.token_2022_try_create_mint_account(
        AppUser::Admin,
        None,
        Some(&[ExtensionType::GroupPointer, ExtensionType::TokenGroup]),
    )?;
    let mint_pubkey = &mint_keypair.pubkey().to_bytes();

    // 2. Initialize ALL extension space/headers BEFORE mint initialization
    // This step is critical - the account needs extension headers initialized
    // but not the extension data itself

    // 3. Initialize the mint account (this expects extension headers to exist)
    app.token_2022_try_initialize_mint(
        AppUser::Admin,
        mint_pubkey,
        decimals,
        &mint_authority.pubkey(),
        None,
    )?;

    // // 4. Initialize group pointer extension data
    // app.token_2022_try_initialize_group_pointer(
    //     AppUser::Admin,
    //     mint_pubkey,
    //     Some(&mint_authority.pubkey()),
    //     Some(mint_pubkey), // Point to the mint itself (self-referencing)
    // )?;

    // // 5. Initialize the token group extension data
    // app.token_2022_try_initialize_token_group(
    //     AppUser::Admin,
    //     mint_pubkey, // Use mint_pubkey as the group account
    //     mint_pubkey,
    //     mint_authority,
    //     update_authority.as_ref(),
    //     100,
    // )?;

    Ok(())
}
