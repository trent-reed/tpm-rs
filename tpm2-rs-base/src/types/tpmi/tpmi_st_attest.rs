use super::*;

/// TpmiStAttest represents an attestation structure type (TPMI_ST_ATTEST).
/// See definition in Part 2: Structures, section 10.12.10.
#[open_enum]
#[repr(u16)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, PartialEq, Default, Marshalable)]
pub enum TpmiStAttest {
    AttestCertify = TPM2ST::AttestCertify.0,
    AttestQuote = TPM2ST::AttestQuote.0,
    AttestSessionAudit = TPM2ST::AttestSessionAudit.0,
    AttestCommandAudit = TPM2ST::AttestCommandAudit.0,
    AttestTime = TPM2ST::AttestTime.0,
    AttestCreation = TPM2ST::AttestCreation.0,
    AttestNV = TPM2ST::AttestNV.0,
    AttestNVDigest = TPM2ST::AttestNVDigest.0,
}
