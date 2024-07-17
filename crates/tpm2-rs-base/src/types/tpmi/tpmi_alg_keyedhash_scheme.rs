use super::*;

/// TpmiAlgKeyedhashScheme represents values that may appear in a keyed_hash as the scheme parameter (TPMI_ALG_KEYEDHASH_SCHEME).
/// See definition in Part 2: Structures, section 11.1.19.
#[open_enum]
#[repr(u16)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, PartialEq, Default, Marshalable)]
#[allow(clippy::upper_case_acronyms)]
pub enum TpmiAlgKeyedhashScheme{
    HMAC = TPM2AlgID::HMAC.0,
    XOR = TPM2AlgID::XOR.0,
}
