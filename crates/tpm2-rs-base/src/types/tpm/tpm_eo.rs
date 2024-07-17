use super::*;

// TPM2EO represents a TPM_EO.
// See definition in Part 2: Structures, section 6.8.
#[open_enum]
#[repr(u16)]
pub enum TPM2EO {
    Eq = 0x0000,
    Neq = 0x0001,
    SignedGT = 0x0002,
    UnsignedGT = 0x0003,
    SignedLT = 0x0004,
    UnsignedLT = 0x0005,
    SignedGE = 0x0006,
    UnsignedGE = 0x0007,
    SignedLE = 0x0008,
    UnsignedLE = 0x0009,
    BitSet = 0x000A,
    BitClear = 0x000B,
}
