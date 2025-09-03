#[repr(u8)]
pub enum ExtensionDiscriminator {
    CpiGuard = 34,
    PermanentDelegate = 35,
    GroupPointer = 40,
    GroupMemberPointer = 41,
}
