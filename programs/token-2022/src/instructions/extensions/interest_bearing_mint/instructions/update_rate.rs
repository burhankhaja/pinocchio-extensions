use {
    crate::instructions::{
        extensions::{
            interest_bearing_mint::{
                offset_intereset_bearing_mint_update_rate as OFFSET, InterestBearingMintInstruction,
            },
            ExtensionDiscriminator,
        },
        MAX_MULTISIG_SIGNERS,
    },
    core::{mem::MaybeUninit, slice},
    pinocchio::{
        account_info::AccountInfo,
        cpi::invoke_signed_with_bounds,
        instruction::{AccountMeta, Instruction, Signer},
        program_error::ProgramError,
        pubkey::Pubkey,
        ProgramResult,
    },
};

/// Update the interest rate. Only supported for mints that
/// include the `InterestBearingConfig` extension.
///
/// Accounts expected by this instruction:
///
///   * Single authority
///   0. `[writable]` The mint.
///   1. `[signer]` The mint rate authority.
///
///   * Multisignature authority
///   0. `[writable]` The mint.
///   1. `[]` The mint's multisignature rate authority.
///   2. `..2+M` `[signer]` M signer accounts.
pub struct UpdateRate<'a, 'b> {
    /// Mint Account
    pub mint: &'a AccountInfo,
    /// The rate authority.
    pub authority: &'a AccountInfo,
    /// The new interest rate
    pub rate: i16,
    /// The Signer accounts if `authority` is a multisig
    pub signers: &'a [AccountInfo],
    /// Token Program
    pub token_program: &'b Pubkey,
}

impl UpdateRate<'_, '_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    #[inline(always)]
    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        let &Self {
            mint,
            authority,
            rate,
            signers: multisig_accounts,
            token_program,
        } = self;

        if multisig_accounts.len() > MAX_MULTISIG_SIGNERS {
            return Err(ProgramError::InvalidArgument);
        }

        const UNINIT_ACCOUNT_METAS: MaybeUninit<AccountMeta> = MaybeUninit::<AccountMeta>::uninit();
        let mut account_metas = [UNINIT_ACCOUNT_METAS; 2 + MAX_MULTISIG_SIGNERS];

        unsafe {
            // SAFETY:
            // - `account_metas` is sized to 2 + MAX_MULTISIG_SIGNERS

            // - Index 0 is always present (Mint)
            account_metas
                .get_unchecked_mut(0)
                .write(AccountMeta::writable(mint.key()));

            // - Index 1 is always present (Authority)
            if multisig_accounts.is_empty() {
                account_metas
                    .get_unchecked_mut(1)
                    .write(AccountMeta::readonly_signer(authority.key()));
            } else {
                account_metas
                    .get_unchecked_mut(1)
                    .write(AccountMeta::readonly(authority.key()));
            }
        }

        for (account_meta, signer) in account_metas[2..].iter_mut().zip(multisig_accounts.iter()) {
            account_meta.write(AccountMeta::readonly_signer(signer.key()));
        }

        // build instruction
        let mut buffer = [0u8; OFFSET::END as usize];
        let data = update_rate_instruction_data(&mut buffer, rate);

        let num_accounts = 2 + multisig_accounts.len();

        let instruction = Instruction {
            program_id: token_program,
            data: data,
            accounts: unsafe {
                slice::from_raw_parts(account_metas.as_ptr() as *const AccountMeta, num_accounts)
            },
        };

        // Account info array
        const UNINIT_ACCOUNT_INFOS: MaybeUninit<&AccountInfo> =
            MaybeUninit::<&AccountInfo>::uninit();
        let mut account_infos = [UNINIT_ACCOUNT_INFOS; 2 + MAX_MULTISIG_SIGNERS];

        unsafe {
            // SAFETY:
            // - `account_infos` is sized to 2 + MAX_MULTISIG_SIGNERS
            // - Index 0 is always present
            account_infos.get_unchecked_mut(0).write(mint);
            // - Index 1 is always present
            account_infos.get_unchecked_mut(1).write(authority);
        }

        // Fill signer accounts
        for (account_info, signer) in account_infos[2..].iter_mut().zip(multisig_accounts.iter()) {
            account_info.write(signer);
        }

        invoke_signed_with_bounds::<{ 2 + MAX_MULTISIG_SIGNERS }>(
            &instruction,
            unsafe {
                slice::from_raw_parts(account_infos.as_ptr() as *const &AccountInfo, num_accounts)
            },
            signers,
        )
    }
}

#[inline(always)]
fn update_rate_instruction_data<'a>(buffer: &'a mut [u8], rate: i16) -> &'a [u8] {
    let offset = OFFSET::START as usize;

    // Set discriminators
    buffer[..offset].copy_from_slice(&[
        ExtensionDiscriminator::InterestBearingMint as u8,
        InterestBearingMintInstruction::UpdateRate as u8,
    ]);

    // Set rate at offset [2..4]
    buffer[offset..OFFSET::END as usize].copy_from_slice(&rate.to_le_bytes());

    buffer
}
