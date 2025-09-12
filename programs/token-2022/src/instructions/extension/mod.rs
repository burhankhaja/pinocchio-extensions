pub mod group_pointer;
pub mod token_group;

#[repr(u8)]
pub enum ExtensionDiscriminator {
    GroupPointer = 40,
}
