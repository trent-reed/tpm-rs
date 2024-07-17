use super::*;

/// TpmiAlgKdf represents all of key derivation functions (TPMI_ALG_KDF).
/// See definition in Part 2: Structures, section 9.32.
#[open_enum]
#[repr(u16)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, PartialEq, Default, Marshalable)]
pub enum TpmiAlgKdf {
    MGF1 = TPM2AlgID::MGF1.0,
    KDF1SP80056A = TPM2AlgID::KDF1SP80056A.0,
    KDF2 = TPM2AlgID::KDF2.0,
    KDF1SP800108 = TPM2AlgID::KDF1SP800108.0,
}
