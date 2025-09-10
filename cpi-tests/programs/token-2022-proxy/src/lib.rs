#![allow(unexpected_cfgs)]

use {
    crate::helpers::show,
    pinocchio::{
        account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
        ProgramResult,
    },
    pinocchio_pubkey::declare_id,
    spl_token_2022_interface::{
        extension::{
            default_account_state::instruction::decode_instruction,
            group_pointer::instruction::GroupPointerInstruction,
        },
        instruction::{decode_instruction_type, TokenInstruction},
    },
};

pub mod helpers;
mod instructions;

use instructions as i;

entrypoint!(process_instruction);
declare_id!("4ibrEMW5F6hKnkW4jVedswYv6H6VtwPN6ar6dvXDN1nT");

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let token_instruction = TokenInstruction::unpack(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match token_instruction {
        TokenInstruction::InitializeMint {
            decimals,
            mint_authority,
            freeze_authority,
        } => i::initialize_mint(accounts, decimals, mint_authority, freeze_authority),

        TokenInstruction::GroupPointerExtension => {
            // Remove extension discriminator
            let instruction_data = &instruction_data[1..];

            let group_pointer_ix: GroupPointerInstruction =
                decode_instruction_type(instruction_data)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;

            match group_pointer_ix {
                GroupPointerInstruction::Initialize => {
                    i::group_pointer::initialize(accounts, instruction_data)
                }
                GroupPointerInstruction::Update => {
                    todo!() // i::group_pointer::update(accounts, instruction_data)
                }
            }
        }

        _ => Err(ProgramError::InvalidInstructionData)?,
    }
}
