use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsSessionAuditInfo {
    pub exclusive_session: TpmiYesNo,
    pub session_digest: Tpm2bDigest,
}
