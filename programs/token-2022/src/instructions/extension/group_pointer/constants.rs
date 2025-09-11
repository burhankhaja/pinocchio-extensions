#[repr(u8)]
pub enum ExtensionDiscriminator {
    GroupPointer = 40,
}

#[repr(u8)]
pub enum InstructionDiscriminatorGroupPointer {
    Initialize = 0,
    Update = 1,
}

/// Instruction data layout:
/// - [0]                        : GroupPointerExtension discriminator (1 byte)
/// - [1]                        : Initialize discriminator (1 byte)
/// - [2]                        : authority presence flag (1 byte, u8)
/// - [3..35]                    : authority pubkey (optional, 32 bytes)
/// - [35 or 3]                  : group_address presence flag (1 byte, u8)
/// - [36..68 or 4..36]          : group_address pubkey (optional, 32 bytes)
pub mod offset_group_pointer_initialize {
    pub const INITIAL: u8 = 2;
    // pub const AUTHORITY_PRESENCE_FLAG: u8 = 1;
    pub const AUTHORITY_PUBKEY: u8 = 32;
    // pub const GROUP_ADDRESS_PRESENCE_FLAG: u8 = 1;
    pub const GROUP_ADDRESS_PUBKEY: u8 = 32;
    pub const MAX: u8 = INITIAL
        // + AUTHORITY_PRESENCE_FLAG
        + AUTHORITY_PUBKEY
        // + GROUP_ADDRESS_PRESENCE_FLAG
        + GROUP_ADDRESS_PUBKEY;
}

/// Instruction data layout:
/// -  [0]: instruction GroupPointerExtension discriminator (1 byte, u8)
/// -  [1]: instruction Update discriminator (1 byte, u8)
/// -  [2]: group_address presence flag (1 byte, u8)
/// -  [3..35]: group_address pubkey (optional, 32 bytes)
pub mod offset_group_pointer_update {
    pub const INITIAL: u8 = 2;
    // pub const GROUP_ADDRESS_PRESENCE_FLAG: u8 = 1;
    pub const GROUP_ADDRESS_PUBKEY: u8 = 32;
    pub const MAX: u8 = INITIAL 
    // + GROUP_ADDRESS_PRESENCE_FLAG 
    + GROUP_ADDRESS_PUBKEY;
}
