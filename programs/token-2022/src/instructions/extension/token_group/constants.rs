#[repr(u64)]
pub enum InstructionDiscriminatorTokenGroup {
    InitializeGroup = 288286683834380665, // [121, 113, 108, 39, 54, 51, 0, 4]
    UpdateGroupMaxSize = 7931435946663945580, // [108, 37, 171, 143, 248, 30, 18, 110]
    UpdateGroupAuthority = 14688734194668431777, // [161, 105, 88, 1, 237, 221, 216, 203]
    InitializeMember = 9688630243381616792, // [152, 32, 222, 176, 223, 237, 116, 134]
}

// /// Instruction data layout:
// /// - [0]                        : GroupPointerExtension discriminator (1 byte)
// /// - [1]                        : Initialize discriminator (1 byte)
// /// - [2..34]                    : authority pubkey (32 bytes)
// /// - [34..66]                   : group_address pubkey (32 bytes)
// pub mod offset_token_group_initialize {
//     pub const START: u8 = 2;
//     pub const AUTHORITY_PUBKEY: u8 = 32;
//     pub const GROUP_ADDRESS_PUBKEY: u8 = 32;
//     pub const END: u8 = START + AUTHORITY_PUBKEY + GROUP_ADDRESS_PUBKEY;
// }
