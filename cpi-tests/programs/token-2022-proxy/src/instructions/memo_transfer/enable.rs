use {
    pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult},
    pinocchio_token_2022,
};

pub fn enable(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    // Expected account layout:
    //   [ token_account, authority/owner , ...signers, token_program ]
    //
    // Single authority case:
    //   [ token_account, authority/owner (signer), token_program ]
    //
    // Multisig case:
    //   [ token_account, authority/owner , signer1, signer2, ... signer n, token_program ]

    if accounts.len() < 3 {
        Err(ProgramError::NotEnoughAccountKeys)?;
    }

    let token_account = &accounts[0];
    let authority = &accounts[1];
    let token_program = accounts.last().unwrap(); // dev : since token_program is always the last account
    let signers = &accounts[2..accounts.len() - 1]; // dev : everything between authority and token_program ; note : in Single authority case: [2..2] will result in empty array

    match instruction_data[1] {
        // dev: byte at index 1 = Action Type; 0 = Enable
        0 => pinocchio_token_2022::extension::memo_transfer::Enable {
            token_account,
            authority,
            signers,
            token_program: token_program.key(),
        }
        .invoke(),

        _ => return Err(ProgramError::InvalidInstructionData),
    }
}
