use super::*;

// TPM2NT represents a TPM_NT.
// See definition in Part 2: Structures, section 13.4.
#[open_enum]
#[repr(u8)]
pub enum TPM2NT {
    // contains data that is opaque to the TPM that can only be modified
    // using TPM2_NV_Write().
    Ordinary = 0x0,
    // contains an 8-octet value that is to be used as a counter and can
    // only be modified with TPM2_NV_Increment()
    Counter = 0x1,
    // contains an 8-octet value to be used as a bit field and can only be
    // modified with TPM2_NV_SetBits().
    Bits = 0x2,
    // contains a digest-sized value used like a PCR. The Index can only be
    // modified using TPM2_NV_Extend(). The extend will use the nameAlg of
    // the Index.
    Extend = 0x4,
    // contains pinCount that increments on a PIN authorization failure and
    // a pinLimit
    PinFail = 0x8,
    // contains pinCount that increments on a PIN authorization success and
    // a pinLimit
    PinPass = 0x9,
}
