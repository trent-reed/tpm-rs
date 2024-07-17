use super::*;

/// TpmiAlgSymObject represents all of the symmetric algorithms that may be used as a companion encryption algortihm for an asymmetric object (TPMI_ALG_SYM_OBJECT).
/// See definition in Part 2: Structures, section 9.30.
#[open_enum]
#[repr(u16)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, PartialEq, Default, Marshalable)]
#[allow(clippy::upper_case_acronyms)]
pub enum TpmiAlgSymObject{
    TDES = TPM2AlgID::TDES.0,
    AES = TPM2AlgID::AES.0,
    SM4 = TPM2AlgID::SM4.0,
    Camellia = TPM2AlgID::Camellia.0,
}
