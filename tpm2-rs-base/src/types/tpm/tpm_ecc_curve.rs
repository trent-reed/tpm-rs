use super::*;

// TPM2ECCCurve represents a TPM_ECC_Curve.
// See definition in Part 2: Structures, section 6.4.
#[open_enum]
#[repr(u16)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, Default, Marshalable)]
pub enum TPM2ECCCurve {
    None = 0x0000,
    NistP192 = 0x0001,
    NistP224 = 0x0002,
    NistP256 = 0x0003,
    NistP384 = 0x0004,
    NistP521 = 0x0005,
    BNP256 = 0x0010,
    BNP638 = 0x0011,
    SM2P256 = 0x0020,
}
