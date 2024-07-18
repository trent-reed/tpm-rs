use super::*;

/// TpmiAlgPublic represents all object types (TPMI_ALG_PUBLIC).
/// See definition in Part 2: Structures, section 12.2.2.
#[open_enum]
#[repr(u16)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, PartialEq, Default, Marshalable)]
#[allow(clippy::upper_case_acronyms)]
pub enum TpmiAlgPublic{
    RSA = TPM2AlgID::RSA.0,
    KeyedHash = TPM2AlgID::KeyedHash.0,
    ECC = TPM2AlgID::ECC.0,
    SymCipher = TPM2AlgID::SymCipher.0,
}
