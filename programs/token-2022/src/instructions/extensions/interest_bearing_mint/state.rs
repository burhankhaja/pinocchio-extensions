use pinocchio::pubkey::Pubkey;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InterestBearingMintInstruction {
    Initialize = 0,
    UpdateRate = 1,
}

#[repr(C, packed)]
pub struct InterestBearingConfig {
    /// Authority that can set the interest rate
    rate_authority: Pubkey,
    /// Initialization timestamp
    initialization_timestamp: i64,
    /// Pre-update average rate
    pre_update_average_rate: i16,
    /// Last update timestamp
    last_update_timestamp: i64,
    /// Current interest rate
    current_rate: i16,
}

/// instruction data
/// -  [0]:                        Extension discriminator (1 byte, u8)
/// -  [1]:                        Instruction_type (1 byte, u8)
/// -  [2..34]:                    rate_authority pubkey (32 bytes, optional)
/// -  [34..36]:                   rate (2 bytes, i16)
pub mod offset_intereset_bearing_mint_initialize {
    pub const START: u8 = 2;
    pub const RATE_AUTHORITY: u8 = 32;
    pub const RATE: u8 = 2;
    pub const END: u8 = START + RATE_AUTHORITY + RATE;
}

/// instruction data
/// -  [0]:                         extension discriminator (1 byte, u8)
/// -  [1]:                         instruction_type (1 byte, u8)
/// -  [2..4]:                      rate (2 bytes, i16)
pub mod offset_intereset_bearing_mint_update_rate {
    pub const START: u8 = 2;
    pub const RATE: u8 = 2;
    pub const END: u8 = START + RATE;
}
