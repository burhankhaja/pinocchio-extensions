use {
    pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult},
    pinocchio_token_2022,
    spl_token_2022_interface::instruction::decode_instruction_data,
};

pub fn initialize(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let [mint, token_program] = accounts else {
        Err(ProgramError::InvalidAccountData)?
    };

    let &spl_token_2022_interface::extension::group_pointer::instruction::InitializeInstructionData {
        authority,
        group_address,
    } = decode_instruction_data(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    pinocchio_token_2022::instructions::extension::group_pointer::Initialize {
        mint,
        authority: Option::<solana_address::Address>::from(authority)
            .map(|x| x.to_bytes())
            .as_ref(),
        group_address: Option::<solana_address::Address>::from(group_address)
            .map(|x| x.to_bytes())
            .as_ref(),
        token_program: &token_program.key(),
    }
    .invoke()
}
