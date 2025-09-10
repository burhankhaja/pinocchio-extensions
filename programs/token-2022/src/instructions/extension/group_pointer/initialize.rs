use {
    crate::{
        instructions::extension::group_pointer::{
            offset_group_pointer_initialize as OFFSET, ExtensionDiscriminator,
            InstructionDiscriminatorGroupPointer,
        },
        option_to_flag, write_bytes, UNINIT_BYTE,
    },
    core::slice::from_raw_parts,
    pinocchio::{
        account_info::AccountInfo,
        cpi::invoke_signed,
        instruction::{AccountMeta, Instruction, Signer},
        pubkey::Pubkey,
        ProgramResult,
    },
};

/// Initialize a new mint with a group pointer
///
/// Accounts expected by this instruction:
///
///  0. `[writable]` The mint to initialize.
pub struct Initialize<'a> {
    /// Mint Account
    pub mint: &'a AccountInfo,
    /// Optional authority that can set the group address
    pub authority: Option<&'a Pubkey>,
    /// Optional account address that holds the group
    pub group_address: Option<&'a Pubkey>,
    /// Token Program
    pub token_program: &'a Pubkey,
}

impl Initialize<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    #[inline(always)]
    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        let account_metas = [AccountMeta::writable(self.mint.key())];

        let data = initialize_instruction_data(self.authority, self.group_address);

        let instruction = Instruction {
            program_id: self.token_program,
            accounts: &account_metas,
            data,
        };

        invoke_signed(&instruction, &[self.mint], signers)
    }
}

pub fn initialize_instruction_data<'a>(
    authority: Option<&'a Pubkey>,
    group_address: Option<&'a Pubkey>,
) -> &'a [u8] {
    // Size depends on presence of authority and group_address
    let mut instruction_data = [UNINIT_BYTE; OFFSET::MAX as usize];

    // === Set discriminators ===
    write_bytes(
        &mut instruction_data,
        &[
            ExtensionDiscriminator::GroupPointer as u8,
            InstructionDiscriminatorGroupPointer::Initialize as u8,
        ],
    );
    let mut offset = OFFSET::INITIAL as usize;

    // === Set authority ===
    // Set option
    write_bytes(
        &mut instruction_data[offset..offset + OFFSET::AUTHORITY_PRESENCE_FLAG as usize],
        &[option_to_flag(authority)],
    );
    offset += OFFSET::AUTHORITY_PRESENCE_FLAG as usize;

    // Try set value
    if let Some(x) = authority {
        write_bytes(
            &mut instruction_data[offset..offset + OFFSET::AUTHORITY_PUBKEY as usize],
            x,
        );
        offset += OFFSET::AUTHORITY_PUBKEY as usize;
    }

    // === Set group_address ===
    // Set option
    write_bytes(
        &mut instruction_data[offset..offset + OFFSET::GROUP_ADDRESS_PRESENCE_FLAG as usize],
        &[option_to_flag(group_address)],
    );
    offset += OFFSET::GROUP_ADDRESS_PRESENCE_FLAG as usize;

    // Try set value
    if let Some(x) = group_address {
        write_bytes(
            &mut instruction_data[offset..offset + OFFSET::GROUP_ADDRESS_PUBKEY as usize],
            x,
        );
        offset += OFFSET::GROUP_ADDRESS_PUBKEY as usize;
    }

    unsafe { from_raw_parts(instruction_data.as_ptr() as _, offset) }
}
