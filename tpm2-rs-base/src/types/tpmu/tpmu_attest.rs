use super::*;

#[repr(C, u16)]
#[derive(Clone, Copy, PartialEq, Marshalable)]
pub enum TpmuAttest {
    Certify(TpmsCertifyInfo) = TPM2ST::AttestCertify.0,
    Creation(TpmsCreationInfo) = TPM2ST::AttestCreation.0,
    Quote(TpmsQuoteInfo) = TPM2ST::AttestQuote.0,
    CommandAudit(TpmsCommandAuditInfo) = TPM2ST::AttestCommandAudit.0,
    SessionAudit(TpmsSessionAuditInfo) = TPM2ST::AttestSessionAudit.0,
    Time(TpmsTimeAttestInfo) = TPM2ST::AttestTime.0,
    Nv(TpmsNvCertifyInfo) = TPM2ST::AttestNV.0,
}
