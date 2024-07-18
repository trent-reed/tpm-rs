// TODO: I haven't looked at this in-depth yet.

use arrayvec::ArrayVec;
use tpm2_rs_errors::TssResult;
use tpm2_rs_base::{
    Tpm2bAuth, Tpm2bNonce, Tpm2bSimple, TpmaSession, TpmiShAuthSession, TpmsAuthCommand,
    TpmsAuthResponse,
};
use tpm2_rs_errors::{TssTcsError, TssErrorCode};

#[cfg(test)]
mod tests;

/// Trait for types representing TPM sessions.
pub trait Session {
    /// Computes the authorization HMAC for this session.
    fn get_auth_command(&self) -> TpmsAuthCommand;
    /// Validates the authorization response for this session.
    fn validate_auth_response(&self, auth: &TpmsAuthResponse) -> TssResult<()>;
}

/// Container for sessions associated with a TPM command. A command can have up to three sessions.
pub type CmdSessions<'a> = ArrayVec<&'a mut dyn Session, 3>;

/// A password session.
#[derive(Debug, PartialEq, Default)]
pub struct PasswordSession {
    auth: Tpm2bAuth,
}

impl Session for PasswordSession {
    fn get_auth_command(&self) -> TpmsAuthCommand {
        TpmsAuthCommand {
            session_handle: TpmiShAuthSession::RS_PW,
            nonce: Tpm2bNonce::default(),
            session_attributes: TpmaSession(0),
            hmac: self.auth,
        }
    }
    fn validate_auth_response(&self, auth: &TpmsAuthResponse) -> TssResult<()> {
        // Password response auth should have empty nonce/hmac and ContinueSession attribute.
        if auth.nonce.get_size() != 0
            || auth.session_attributes.0 != 0x1
            || auth.hmac.get_size() != 0
        {
            Err(TssTcsError::new(TssErrorCode::BadParameter).into())
        } else {
            Ok(())
        }
    }
}
