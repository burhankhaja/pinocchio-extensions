use {
    crate::instructions::extensions::{
        interest_bearing_mint::{
            offset_intereset_bearing_mint_initialize as OFFSET, InterestBearingMintInstruction,
        },
        ExtensionDiscriminator,
    },
    core::slice,
    pinocchio::{
        account_info::AccountInfo,
        cpi::invoke_signed,
        instruction::{AccountMeta, Instruction, Signer},
        pubkey::Pubkey,
        ProgramResult,
    },
};

/// Initialize a new mint with interest accrual
///
/// Accounts expected by this instruction:
///
///  0. `[writable]` The mint to initialize.
pub struct Initialize<'a, 'b> {
    /// Mint Account
    pub mint: &'a AccountInfo,
    /// Optional authority that can set the interest rate
    pub rate_authority: Option<&'b Pubkey>,
    /// The initial interest rate
    pub rate: i16,
    /// Token Program
    pub token_program: &'b Pubkey,
}

impl Initialize<'_, '_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    #[inline(always)]
    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        let &Self {
            mint,
            rate_authority,
            rate,
            token_program,
        } = self;

        let account_metas = [AccountMeta::writable(mint.key())];

        let mut buffer = [0u8; OFFSET::END as usize];
        let data = initialize_instruction_data(&mut buffer, rate_authority, rate);

        let instruction = Instruction {
            program_id: token_program,
            accounts: &account_metas,
            data: unsafe { slice::from_raw_parts(data.as_ptr() as _, data.len()) },
        };

        invoke_signed(&instruction, &[mint], signers)
    }
}

#[inline(always)]
fn initialize_instruction_data<'a>(
    buffer: &'a mut [u8],
    rate_authority: Option<&Pubkey>,
    rate: i16,
) -> &'a [u8] {
    let mut offset = OFFSET::START as usize;

    // Set discriminators
    buffer[..offset].copy_from_slice(&[
        ExtensionDiscriminator::InterestBearingMint as u8,
        InterestBearingMintInstruction::Initialize as u8,
    ]);

    // Set rate_authority at offset [2..34]
    if let Some(auth) = rate_authority {
        buffer[offset..offset + OFFSET::RATE_AUTHORITY as usize].copy_from_slice(auth);
    } else {
        buffer[offset..offset + OFFSET::RATE_AUTHORITY as usize].copy_from_slice(&[0u8; 32]);
    }

    // shift offset to 34
    offset += OFFSET::RATE_AUTHORITY as usize;

    // Set rate at offset [34..36]
    buffer[offset..OFFSET::END as usize].copy_from_slice(&rate.to_le_bytes());

    buffer
}
