use super::*;

#[derive(Clone, Copy, PartialEq, Marshalable)]
pub struct TpmsCommandAuditInfo {
    pub audit_counter: u64,
    pub digest_alg: u16,
    pub audit_digest: Tpm2bDigest,
    pub command_digest: Tpm2bDigest,
}
