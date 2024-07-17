use super::*;

// TPM2SU represents a TPM_SU.
// See definition in Part 2: Structures, section 6.10.
#[open_enum]
#[repr(u16)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, Default, Marshalable)]
pub enum TPM2SU {
    Clear = 0x0000,
    State = 0x0001,
}
