use core::mem::MaybeUninit;
use crate::{write_bytes, UNINIT_BYTE, ID};
use crate::extension::consts::PAUSABLE_EXTENSION;
use pinocchio::{
    account_info::{AccountInfo, Ref},
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PausableInstruction {
    Initialize,
    Pause,
    Resume,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PausableConfig {
    /// Authority
    authority: Pubkey,
    /// Whether token is paused
    paused: u8,
}

impl PausableConfig {
    /// The length of the mint with `PausableConfig` extension data
    const LEN: u8 = 117;
    /// The index where pausable config data starts in the mint with `PausableConfig` extension data
    const PAUSABLE_CONFIG_START: u8 = 84;

    /// The length of the `PausableConfig` extension data.
    pub const BASE_LEN: usize = core::mem::size_of::<PausableConfig>();

    /// Return a `PausableConfig` from the given account info.
    ///
    /// This method performs owner and length validation on `AccountInfo`, safe borrowing
    /// the account data.
    #[inline]
    pub fn from_account_info(
        account_info: &AccountInfo,
    ) -> Result<Ref<PausableConfig>, ProgramError> {
        // Check data length first
        if account_info.data_len() < Self::LEN as usize {
            Err(ProgramError::InvalidAccountData)?;
        }

        // Check owner
        if account_info.owner() != &ID {
            Err(ProgramError::InvalidAccountOwner)?;
        }

        // Safely borrow and map the data
        let data_ref = account_info
            .try_borrow_data()
            .map_err(|_| ProgramError::AccountBorrowFailed)?;

        Ok(Ref::map(data_ref, |data| unsafe {
            Self::from_bytes_unchecked(data)
        }))
    }

    /// Return a `PausableConfig` from the given account info.
    ///
    /// This method performs owner and length validation on `AccountInfo`, but does not
    /// perform the borrow check.
    ///
    /// # Safety
    ///
    /// The caller must ensure that it is safe to borrow the account data (e.g., there are
    /// no mutable borrows of the account data).
    #[inline]
    pub unsafe fn from_account_info_unchecked(
        account_info: &AccountInfo,
    ) -> Result<&Self, ProgramError> {
        // Check data length first
        if account_info.data_len() < Self::LEN as usize {
            Err(ProgramError::InvalidAccountData)?;
        }

        // Check owner
        if account_info.owner() != &ID {
            Err(ProgramError::InvalidAccountOwner)?;
        }

        // Get unchecked borrow and convert
        let data = account_info.borrow_data_unchecked();
        Ok(Self::from_bytes_unchecked(data))
    }

    /// Return a `PausableConfig` from the given bytes.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// 1. `bytes` contains at least `LEN` bytes
    /// 2. `bytes` contains a valid representation of `PausableConfig`
    /// 3. The data is properly aligned
    #[inline(always)]
    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Self {
        &*(bytes[Self::PAUSABLE_CONFIG_START as usize..].as_ptr() as *const PausableConfig)
    }

    /// Safe version of from_bytes that performs validation
    #[inline]
    pub fn from_bytes(bytes: &[u8]) -> Result<&Self, ProgramError> {
        if bytes.len() < Self::LEN as usize {
            Err(ProgramError::InvalidAccountData)?;
        }

        Ok(unsafe { Self::from_bytes_unchecked(bytes) })
    }

    /// Check if the mint has an authority set
    #[inline(always)]
    pub fn has_authority(&self) -> bool {
        self.authority != Pubkey::default()
    }

    /// Get the authority if it exists
    #[inline]
    pub fn authority(&self) -> Option<&Pubkey> {
        if self.has_authority() {
            Some(&self.authority)
        } else {
            None
        }
    }

    /// Get the authority without checking if it exists
    ///
    /// This method should be used when the caller knows that the pausable config will have an
    /// authority set since it skips the `Option` check.
    #[inline(always)]
    pub fn authority_unchecked(&self) -> &Pubkey {
        &self.authority
    }

    /// Check if the mint is paused
    #[inline(always)]
    pub fn is_paused(&self) -> bool {
        self.paused != 0
    }
}

pub fn pausable_instruction_data(
    instruction_type: PausableInstruction,
) -> [MaybeUninit<u8>; 2] {
    // instruction data
    // -  [0]: instruction discriminator (1 byte, u8)
    // -  [1]: instruction_type (1 byte, u8)
    
    let mut data = [UNINIT_BYTE; 2];
    // Set extension discriminator at offset [0]
    write_bytes(&mut data, &[PAUSABLE_EXTENSION]);
    // Set sub-instruction at offset [1]
    write_bytes(&mut data[1..2], &[instruction_type as u8]);

    data
}

pub fn pausable_initialize_instruction_data(
    instruction_type: PausableInstruction,
    authority: Option<&Pubkey>,
) -> [MaybeUninit<u8>; 34] {
    // instruction data
    // -  [0]: instruction discriminator (1 byte, u8)
    // -  [1]: instruction_type (1 byte, u8)
    // -  [2..34]: authority (32 bytes, Pubkey) - can be zero if None
    
    let mut data = [UNINIT_BYTE; 34];
    // Set extension discriminator at offset [0]
    write_bytes(&mut data, &[PAUSABLE_EXTENSION]);
    // Set sub-instruction at offset [1]
    write_bytes(&mut data[1..2], &[instruction_type as u8]);
    // Set authority at offset [2..34]
    if let Some(auth) = authority {
        write_bytes(&mut data[2..34], auth);
    } else {
        write_bytes(&mut data[2..34], &[0u8; 32]);
    }

    data
}
