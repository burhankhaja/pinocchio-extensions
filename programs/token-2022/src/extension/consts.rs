#[repr(u8)]
pub enum ExtensionDiscriminator {
    CpiGuard = 34,
    PermanentDelegate = 35,
    ScaledUiAmount = 0,
    Pausable = 36,
    DefaultAccountState = 28,
    GroupPointer = 40,
    GroupMemberPointer = 41,
}
