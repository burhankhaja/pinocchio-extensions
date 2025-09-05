use {
    crate::serde::{deserialize, from_pod_c_option},
    bytemuck::{Pod, Zeroable},
    pinocchio::{
        account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
    },
    pinocchio_token_2022,
    spl_token_2022_interface::pod::PodCOption,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct InstructionData {
    pub decimals: u8,
    pub mint_authority: Pubkey,
    pub freeze_authority: PodCOption<Pubkey>,
    pub token_program: Pubkey,
}

pub fn initialize_mint(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let [mint, rent_sysvar] = accounts else {
        Err(ProgramError::InvalidAccountData)?
    };

    let InstructionData {
        decimals,
        mint_authority,
        freeze_authority,
        token_program,
    } = deserialize(instruction_data)?;

    pinocchio_token_2022::instructions::InitializeMint {
        mint,
        rent_sysvar,
        decimals: *decimals,
        mint_authority,
        freeze_authority: from_pod_c_option(freeze_authority).as_ref(),
        token_program,
    }
    .invoke()
}
