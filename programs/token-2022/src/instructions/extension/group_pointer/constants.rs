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
/// - [2..34]                    : authority pubkey (32 bytes)
/// - [34..66]                   : group_address pubkey (32 bytes)
pub mod offset_group_pointer_initialize {
    pub const START: u8 = 2;
    pub const AUTHORITY_PUBKEY: u8 = 32;
    pub const GROUP_ADDRESS_PUBKEY: u8 = 32;
    pub const END: u8 = START + AUTHORITY_PUBKEY + GROUP_ADDRESS_PUBKEY;
}

/// Instruction data layout:
/// -  [0]: instruction GroupPointerExtension discriminator (1 byte, u8)
/// -  [1]: instruction Update discriminator (1 byte, u8)
/// -  [2..34]: group_address pubkey (optional, 32 bytes)
pub mod offset_group_pointer_update {
    pub const START: u8 = 2;
    pub const GROUP_ADDRESS_PUBKEY: u8 = 32;
    pub const END: u8 = START + GROUP_ADDRESS_PUBKEY;
}
