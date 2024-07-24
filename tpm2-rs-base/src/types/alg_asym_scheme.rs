use super::*;

/// TpmiAlgAsymScheme represents all the scheme types for any asymmetric algortihm (TPMI_ALG_ASYM_SCHEME).
/// See definition in Part 2: Structures, section 11.2.3.4.
#[open_enum]
#[repr(u16)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, PartialEq, Default, Marshalable)]
#[allow(clippy::upper_case_acronyms)]
pub enum TpmiAlgAsymScheme{
    SM2 = TPM2AlgID::SM2.0,
    ECDH = TPM2AlgID::ECDH.0,
    ECMQV = TPM2AlgID::ECMQV.0,
    RSAPSS = TPM2AlgID::RSAPSS.0,
    RSASSA = TPM2AlgID::RSASSA.0,
    ECDSA = TPM2AlgID::ECDSA.0,
    ECDAA = TPM2AlgID::ECDAA.0,
    ECSchnorr = TPM2AlgID::ECSchnorr.0,
    RSAES = TPM2AlgID::RSAES.0,
    OAEP = TPM2AlgID::OAEP.0,
}
