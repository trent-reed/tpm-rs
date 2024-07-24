use super::*;

// TPM2Handle represents a TPM_HANDLE.
// See definition in Part 2: Structures, section 7.1.
#[open_enum]
#[repr(u32)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, Default, Marshalable)]
pub enum TPM2Handle {
    RHOwner = 0x40000001,
    RHNull = 0x40000007,
    RSPW = 0x40000009,
    RHLockout = 0x4000000A,
    RHEndorsement = 0x4000000B,
    RHPlatform = 0x4000000C,
    RHPlatformNV = 0x4000000D,
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmlHandle {
    count: u32,
    #[marshal(length=count)]
    handle: [TPM2Handle; TPM2_MAX_CAP_HANDLES],
}

impl_tpml! {TpmlHandle, handle, TPM2Handle, TPM2_MAX_CAP_HANDLES}
