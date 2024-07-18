use super::*;

/// TpmiAlgEccScheme represents values that may appear in the scheme parameter of a TpmtEccScheme (TPMI_ALG_ECC_SCHEME).
/// See definition in Part 2: Structures, section 11.2.5.4.
#[open_enum]
#[repr(u16)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, PartialEq, Default, Marshalable)]
#[allow(clippy::upper_case_acronyms)]
pub enum TpmiAlgEccScheme{
    RSAPSS = TPM2AlgID::RSAPSS.0,
    RSASSA = TPM2AlgID::RSASSA.0,
    ECDSA = TPM2AlgID::ECDSA.0,
    ECDAA = TPM2AlgID::ECDAA.0,
    SM2 = TPM2AlgID::SM2.0,
    ECSchnorr = TPM2AlgID::ECSchnorr.0,
    ECDH = TPM2AlgID::ECDH.0,
    ECMQV = TPM2AlgID::ECMQV.0,
}
