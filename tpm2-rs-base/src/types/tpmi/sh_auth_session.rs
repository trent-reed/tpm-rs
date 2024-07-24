use super::*;

/// TpmiShAuthSessions represents handles referring to an authorization session (TPMI_SH_AUTH_SESSION).
/// See definition in Part 2: Structures, section 9.8.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug, Default, Marshalable)]
pub struct TpmiShAuthSession(u32);
impl TryFrom<u32> for TpmiShAuthSession {
    type Error = TssTspError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if TpmHc::is_hmac_session(value)
            || TpmHc::is_policy_session(value)
            || (value == Self::RS_PW.0)
        {
            Ok(TpmiShAuthSession(value))
        } else {
            Err(TssTspError::new(TssErrorCode::BadParameter))
        }
    }
}
impl TpmiShAuthSession {
    /// A password authorization.
    pub const RS_PW: TpmiShAuthSession = TpmiShAuthSession(TPM2Handle::RSPW.0);
}
