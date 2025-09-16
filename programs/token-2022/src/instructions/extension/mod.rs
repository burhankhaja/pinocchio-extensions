pub mod group_member_pointer;
pub mod group_pointer;
pub mod permanent_delegate;
pub mod token_group;

#[repr(u8)]
pub enum ExtensionDiscriminator {
    PermanentDelegate = 35,
    GroupPointer = 40,
    GroupMemberPointer = 41,
}
