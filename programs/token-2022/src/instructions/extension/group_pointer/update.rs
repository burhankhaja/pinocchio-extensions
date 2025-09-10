use {
    crate::{
        instructions::{
            extension::group_pointer::{
                offset_group_pointer_update as OFFSET, ExtensionDiscriminator,
                InstructionDiscriminatorGroupPointer,
            },
            MAX_MULTISIG_SIGNERS,
        },
        option_to_flag, write_bytes, UNINIT_BYTE,
    },
    core::{
        mem::MaybeUninit,
        slice::{self, from_raw_parts},
    },
    pinocchio::{
        account_info::AccountInfo,
        cpi::invoke_signed_with_bounds,
        instruction::{AccountMeta, Instruction, Signer},
        program_error::ProgramError,
        pubkey::Pubkey,
        ProgramResult,
    },
};

/// Update the group pointer address. Only supported for mints that
/// include the `GroupPointer` extension.
///
/// Accounts expected by this instruction:
///
///   * Single authority
///   0. `[writable]` The mint.
///   1. `[signer]` The group pointer authority.
///
///   * Multisignature authority
///   0. `[writable]` The mint.
///   1. `[]` The mint's group pointer authority.
///   2. `..2+M` `[signer]` M signer accounts.
pub struct Update<'a> {
    /// Mint Account
    pub mint: &'a AccountInfo,
    /// The group pointer authority.
    pub authority: &'a AccountInfo,
    /// The new account address that holds the group
    pub group_address: Option<&'a Pubkey>,
    /// The Signer accounts if `authority` is a multisig
    pub signers: &'a [&'a AccountInfo],
    /// Token Program
    pub token_program: &'a Pubkey,
}

impl Update<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    #[inline(always)]
    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        let &Self {
            mint,
            authority,
            group_address,
            signers: account_signers,
            token_program,
        } = self;

        if account_signers.len() > MAX_MULTISIG_SIGNERS {
            Err(ProgramError::InvalidArgument)?;
        }

        let num_accounts = 2 + account_signers.len();

        // Account metadata
        const UNINIT_META: MaybeUninit<AccountMeta> = MaybeUninit::<AccountMeta>::uninit();
        let mut acc_metas = [UNINIT_META; 2 + MAX_MULTISIG_SIGNERS];

        unsafe {
            // SAFETY:
            // - `account_metas` is sized to 2 + MAX_MULTISIG_SIGNERS
            // - Index 0 is always present
            acc_metas
                .get_unchecked_mut(0)
                .write(AccountMeta::writable(mint.key()));
            // - Index 1 is always present
            if account_signers.is_empty() {
                acc_metas
                    .get_unchecked_mut(1)
                    .write(AccountMeta::readonly_signer(authority.key()));
            } else {
                acc_metas
                    .get_unchecked_mut(1)
                    .write(AccountMeta::readonly(authority.key()));
            }
        }

        for (account_meta, signer) in acc_metas[2..].iter_mut().zip(account_signers.iter()) {
            account_meta.write(AccountMeta::readonly_signer(signer.key()));
        }

        let data = update_instruction_data(group_address);

        let instruction = Instruction {
            program_id: token_program,
            accounts: unsafe { slice::from_raw_parts(acc_metas.as_ptr() as _, num_accounts) },
            data,
        };

        // Account info array
        const UNINIT_INFO: MaybeUninit<&AccountInfo> = MaybeUninit::uninit();
        let mut acc_infos = [UNINIT_INFO; 2 + MAX_MULTISIG_SIGNERS];

        unsafe {
            // SAFETY:
            // - `account_infos` is sized to 2 + MAX_MULTISIG_SIGNERS
            // - Index 0 is always present
            acc_infos.get_unchecked_mut(0).write(mint);
            // - Index 1 is always present
            acc_infos.get_unchecked_mut(1).write(authority);
        }

        // Fill signer accounts
        for (account_info, signer) in acc_infos[2..].iter_mut().zip(account_signers.iter()) {
            account_info.write(signer);
        }

        invoke_signed_with_bounds::<{ 2 + MAX_MULTISIG_SIGNERS }>(
            &instruction,
            unsafe { slice::from_raw_parts(acc_infos.as_ptr() as _, num_accounts) },
            signers,
        )
    }
}

pub fn update_instruction_data<'a>(group_address: Option<&'a Pubkey>) -> &'a [u8] {
    // Size depends on presence of authority and group_address
    let mut instruction_data = [UNINIT_BYTE; OFFSET::MAX as usize];

    // === Set discriminators ===
    write_bytes(
        &mut instruction_data,
        &[
            ExtensionDiscriminator::GroupPointer as u8,
            InstructionDiscriminatorGroupPointer::Update as u8,
        ],
    );
    let mut offset = OFFSET::INITIAL as usize;

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
