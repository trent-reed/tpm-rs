use super::*;

/// TpmiAlgRsaScheme represents values that may appear in the scheme parameter of a TpmsRsaParms (TPMI_ALG_RSA_SCHEME).
/// See definition in Part 2: Structures, section 11.2.4.1.
#[open_enum]
#[repr(u16)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, PartialEq, Default, Marshalable)]
#[allow(clippy::upper_case_acronyms)]
pub enum TpmiAlgRsaScheme{
    RSAPSS = TPM2AlgID::RSAPSS.0,
    RSASSA = TPM2AlgID::RSASSA.0,
    ECDSA = TPM2AlgID::ECDSA.0,
    ECDAA = TPM2AlgID::ECDAA.0,
    SM2 = TPM2AlgID::SM2.0,
    ECSchnorr = TPM2AlgID::ECSchnorr.0,
    RSAES = TPM2AlgID::RSAES.0,
    OAEP = TPM2AlgID::OAEP.0,
}
