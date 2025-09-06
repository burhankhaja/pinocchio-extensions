use {
    crate::serde::{from_c_option, show},
    pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult},
    pinocchio_token_2022,
    solana_address::Address,
    solana_program_option::COption,
};

pub fn initialize_mint(
    accounts: &[AccountInfo],
    decimals: u8,
    mint_authority: Address,
    freeze_authority: COption<Address>,
) -> ProgramResult {
    let [mint, rent_sysvar] = accounts else {
        Err(ProgramError::InvalidAccountData)?
    };

    show("decimals", &decimals);
    show("mint_authority", &mint_authority);
    show("freeze_authority_raw", &freeze_authority);

    let freeze_authority = from_c_option(freeze_authority).map(|x| x.to_bytes());
    show("freeze_authority", &freeze_authority);

    // TODO: deploy "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" (Token-2022) as it doesn't exist in test env

    show("invoke", "");
    pinocchio_token_2022::instructions::InitializeMint {
        mint,
        rent_sysvar,
        decimals,
        mint_authority: &mint_authority.to_bytes(),
        freeze_authority: freeze_authority.as_ref(),
        token_program: &spl_token_2022_interface::ID.to_bytes(),
    }
    .invoke()
}
