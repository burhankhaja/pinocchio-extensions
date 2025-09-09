use {
    crate::helpers::{
        extensions::token_2022::{
            initialize_mint::Token2022InitializeMintExtension,
            token_group::Token2022TokenGroupExtension,
        },
        suite::{
            core::App,
            types::{
                pin_pubkey_to_addr, pin_to_sol_pubkey, to_c_option, AppUser, PinPubkey, TestError,
                TestResult,
            },
        },
    },
    pretty_assertions::assert_eq,
    solana_keypair::Keypair,
    solana_program_option::COption,
    solana_signer::Signer,
    spl_pod::{bytemuck::pod_from_bytes, optional_keys::OptionalNonZeroPubkey},
    spl_token_2022_interface::extension::{BaseStateWithExtensions, ExtensionType},
    std::io::Read,
};

// ++
// #[test]
// fn initialize_group_pointer_without_token_group() -> TestResult<()> {
//     let mut app = App::new(true);

//     let decimals: u8 = 6;
//     let mint_authority = AppUser::Admin;
//     let freeze_authority = Some(AppUser::Admin.pubkey());
//     let update_authority = Some(AppUser::Admin.pubkey());

//     // 1. Create mint account with GroupPointer extension
//     let (_, mint_keypair) = app.token_2022_try_create_mint_account(
//         AppUser::Admin,
//         None,
//         Some(&[ExtensionType::GroupPointer]),
//     )?;
//     let mint_pubkey = &mint_keypair.pubkey().to_bytes();

//     println!(
//         "mint_address {:#?}",
//         solana_pubkey::Pubkey::new_from_array(*mint_pubkey)
//     );
//     println!("mint {:#?}", app.token_2022_query_mint_state(mint_pubkey)?);

//     app.token_2022_try_initialize_group_pointer(
//         AppUser::Admin,
//         mint_pubkey,
//         Some(&mint_authority.pubkey()),
//         None,
//     )?;

//     app.token_2022_try_initialize_mint(
//         AppUser::Admin,
//         mint_pubkey,
//         decimals,
//         &mint_authority.pubkey(),
//         freeze_authority.as_ref(),
//     )?;

//     println!(
//         "pointer {:#?}",
//         app.token_2022_query_group_pointer_state(mint_pubkey)?
//     );

//     // println!(
//     //     "group {:#?}",
//     //     app.token_2022_query_token_group_state(mint_pubkey)?
//     // );

//     Ok(())
// }

// ++
// #[test]
// fn initialize_group_pointer_with_fake_token_group() -> TestResult<()> {
//     let mut app = App::new(true);

//     let decimals: u8 = 6;
//     let mint_authority = AppUser::Admin;
//     let freeze_authority = Some(AppUser::Admin.pubkey());
//     let update_authority = Some(AppUser::Admin.pubkey());

//     // 1. Create mint account with GroupPointer extension
//     let (_, mint_keypair) = app.token_2022_try_create_mint_account(
//         AppUser::Admin,
//         None,
//         Some(&[ExtensionType::GroupPointer]),
//     )?;
//     let mint_pubkey = &mint_keypair.pubkey().to_bytes();

//     println!(
//         "mint_address {:#?}",
//         solana_pubkey::Pubkey::new_from_array(*mint_pubkey)
//     );
//     println!("mint {:#?}", app.token_2022_query_mint_state(mint_pubkey)?);

//     let group_keypair = Keypair::new();
//     let group_pubkey = &group_keypair.pubkey().to_bytes();

//     app.token_2022_try_initialize_group_pointer(
//         AppUser::Admin,
//         mint_pubkey,
//         Some(&mint_authority.pubkey()),
//         Some(group_pubkey),
//     )?;

//     app.token_2022_try_initialize_mint(
//         AppUser::Admin,
//         mint_pubkey,
//         decimals,
//         &mint_authority.pubkey(),
//         freeze_authority.as_ref(),
//     )?;

//     println!(
//         "group_address {:#?}",
//         solana_pubkey::Pubkey::new_from_array(*group_pubkey)
//     );
//     println!(
//         "pointer {:#?}",
//         app.token_2022_query_group_pointer_state(mint_pubkey)?
//     );

//     Ok(())
// }

#[test]
fn initialize_group_pointer_with_token_group() -> TestResult<()> {
    let mut app = App::new(false);

    let decimals: u8 = 6;
    let mint_authority = AppUser::Admin;
    let freeze_authority = Some(AppUser::Admin.pubkey());
    let update_authority = pin_pubkey_to_addr(&AppUser::Admin.pubkey());

    // 1. Create mint account with GroupPointer extension
    let (_, mint_keypair) = app.token_2022_try_create_mint_account(
        AppUser::Admin,
        None,
        Some(&[ExtensionType::GroupPointer]),
    )?;
    let mint_pubkey = &mint_keypair.pubkey().to_bytes();

    println!(
        "mint_address {:#?}",
        solana_pubkey::Pubkey::new_from_array(*mint_pubkey)
    );
    println!("mint {:#?}", app.token_2022_query_mint_state(mint_pubkey)?);

    let group_pubkey = mint_pubkey;

    app.token_2022_try_initialize_group_pointer(
        AppUser::Admin,
        mint_pubkey,
        Some(&mint_authority.pubkey()),
        Some(group_pubkey),
    )?;

    let max_size = 10;

    app.token_2022_try_initialize_mint(
        AppUser::Admin,
        mint_pubkey,
        decimals,
        &mint_authority.pubkey(),
        freeze_authority.as_ref(),
    )?;

    // let token_group = spl_token_group_interface::state::TokenGroup::new(
    //     &pin_pubkey_to_addr(mint_pubkey),
    //     OptionalNonZeroPubkey(update_authority),
    //     max_size,
    // );

    app.token_2022_try_initialize_token_group(
        AppUser::Admin,
        group_pubkey,
        mint_pubkey,
        mint_authority,
        Some(&update_authority.to_bytes()),
        max_size,
    )?;

    println!(
        "group_address {:#?}",
        solana_pubkey::Pubkey::new_from_array(*group_pubkey)
    );
    println!(
        "pointer {:#?}",
        app.token_2022_query_group_pointer_state(mint_pubkey)?
    );

    println!(
        "group {:#?}",
        app.token_2022_query_token_group_state(group_pubkey)?
    );

    Ok(())
}
