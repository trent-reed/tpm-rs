use super::*;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Debug, Marshalable)]
pub struct TpmsAuthResponse {
    pub nonce: Tpm2bNonce,
    pub session_attributes: TpmaSession,
    pub hmac: Tpm2bData,
}
