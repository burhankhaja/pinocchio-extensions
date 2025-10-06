#[repr(u8)]
pub enum ExtensionDiscriminator {
    MemoTransfer = 30,
    PermanentDelegate = 35,
    GroupPointer = 40,
    GroupMemberPointer = 41,
}
