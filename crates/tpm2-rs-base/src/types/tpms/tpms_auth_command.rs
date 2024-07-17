use super::*;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Debug, Marshalable)]
pub struct TpmsAuthCommand {
    pub session_handle: TpmiShAuthSession,
    pub nonce: Tpm2bNonce,
    pub session_attributes: TpmaSession,
    pub hmac: Tpm2bAuth,
}
