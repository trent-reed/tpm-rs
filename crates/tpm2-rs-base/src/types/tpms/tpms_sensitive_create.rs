use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsSensitiveCreate {
    pub user_auth: Tpm2bAuth,
    pub data: Tpm2bSensitiveData,
}
