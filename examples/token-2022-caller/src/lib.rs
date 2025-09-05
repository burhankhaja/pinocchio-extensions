#![no_std]
#![allow(unexpected_cfgs)]

use {
    pinocchio::{
        account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
        ProgramResult,
    },
    pinocchio_pubkey::declare_id,
};

mod instructions;
pub mod serde;

use instructions as i;

entrypoint!(process_instruction);
declare_id!("4ibrEMW5F6hKnkW4jVedswYv6H6VtwPN6ar6dvXDN1nT");

// TODO: generate discriminator enum from spl_token_2022_interface::instruction::TokenInstruction
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        None => Err(ProgramError::InvalidInstructionData),
        Some((discriminator, data)) => {
            let instruction = match *discriminator {
                0 => i::initialize_mint,
                _ => Err(ProgramError::InvalidInstructionData)?,
            };
            instruction(accounts, data)
        }
    }
}
