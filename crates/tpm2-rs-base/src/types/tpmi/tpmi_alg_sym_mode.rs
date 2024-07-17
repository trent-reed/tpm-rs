use super::*;

/// TpmiAlgSymMode represents all of block-cipher modes of operation (TPMI_ALG_SYM_MODE).
/// See definition in Part 2: Structures, section 9.31.
#[open_enum]
#[repr(u16)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, PartialEq, Default, Marshalable)]
#[allow(clippy::upper_case_acronyms)]
pub enum TpmiAlgSymMode{
    CMAC = TPM2AlgID::CMAC.0,
    CTR = TPM2AlgID::CTR.0,
    OFB = TPM2AlgID::OFB.0,
    CBC = TPM2AlgID::CBC.0,
    CFB = TPM2AlgID::CFB.0,
    ECB = TPM2AlgID::ECB.0,
}
