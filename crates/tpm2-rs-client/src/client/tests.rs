use core::ops::Deref;
use core::ops::DerefMut;

use crate::sessions::{PasswordSession, Session};

use super::*;
use tpm2_rs_base::commands::TpmCommand;
use tpm2_rs_base::TPM2Handle;
use tpm2_rs_base::TpmaSession;
use tpm2_rs_base::TPM2CC;
use tpm2_rs_base::TPM2ST;
use tpm2_rs_errors::*;

// =============================================================================
// Helper Types
// =============================================================================

#[derive(Default)]
struct StubTpmConnection {}
impl TpmConnection for StubTpmConnection {
    fn transact(&mut self, _request: &[u8], _response: &mut [u8]) -> TssResult<()> {
        Ok(())
    }
}

// Convenience to access the mock connection within a client.
impl Deref for TpmClient<MockTpmConnection> {
    type Target = MockTpmConnection;
    fn deref(&self) -> &Self::Target {
        &self.connection
    }
}
impl DerefMut for TpmClient<MockTpmConnection> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.connection
    }
}

// =============================================================================
// Tests
// =============================================================================

// Larger than the maximum size.
#[derive(Marshalable)]
struct HugeFakeCommand([u8; CMD_BUFFER_SIZE]);
impl TpmCommand for HugeFakeCommand {
    const CMD_CODE: TPM2CC = TPM2CC::NVUndefineSpaceSpecial;
    type Handles = ();
    type RespT = u8;
    type RespHandles = ();
}

#[test]
fn test_command_too_large() {
    let mut client = TpmClient::new(StubTpmConnection::default());
    let too_large = HugeFakeCommand([0; CMD_BUFFER_SIZE]);
    assert_eq!(client.run_command(&too_large), Err(TPM_RC_MEMORY.into()));
}

#[derive(Marshalable)]
struct TestCommand(u32);
impl TpmCommand for TestCommand {
    const CMD_CODE: TPM2CC = TPM2CC::NVUndefineSpaceSpecial;
    type Handles = ();
    type RespT = u32;
    type RespHandles = ();
}

#[test]
fn test_tpm_error() {
    let mut client = TpmClient::new(MockTpmConnection::default());
    client
        .expect_transact()
        .return_const(Err(TssTcsError::new(TssErrorCode::Fail).into()));
    let cmd = TestCommand(56789);
    assert_eq!(
        client.run_command(&cmd),
        Err(TssTcsError::new(TssErrorCode::Fail).into())
    );
}

// TpmLoopbackConnection reads/stores the command header and a u32 "command".
// It responds with a response header and the same u32 "response".
// TODO: Not a big fan of this as a test, we should probably do this differently.
struct TpmLoopbackConnection {
    rxed_header: Option<CmdHeader>,
    rxed_bytes: usize,
}
impl TpmConnection for TpmLoopbackConnection {
    fn transact(&mut self, command: &[u8], response: &mut [u8]) -> TssResult<()> {
        self.rxed_bytes = command.len();
        let mut buf = UnmarshalBuf::new(command);
        self.rxed_header = Some(CmdHeader::try_unmarshal(&mut buf)?);
        let rxed_value = u32::try_unmarshal(&mut buf)?;

        let mut tx_header = RespHeader {
            tag: TPM2ST::NoSessions,
            size: 0,
            rc: 0,
        };
        let mut written = tx_header.try_marshal(response)?;
        written += rxed_value.try_marshal(&mut response[written..])?;
        tx_header.size = written as u32;
        // Update the size.
        tx_header.try_marshal(response)?;
        Ok(())
    }
}

#[test]
fn test_fake_command() {
    let mut client = TpmClient::new(TpmLoopbackConnection {
        rxed_header: None,
        rxed_bytes: 0,
    });
    let cmd = TestCommand(56789);
    let result = client.run_command(&cmd);
    assert_eq!(
        client.connection.rxed_header.unwrap().code,
        TestCommand::CMD_CODE
    );
    assert_eq!(result.unwrap(), cmd.0);
}

#[test]
fn test_bad_response_size() {
    let mut client = TpmClient::new(MockTpmConnection::default());
    client.expect_transact().returning(|_, resp: &mut [u8]| {
        RespHeader {
            tag: TPM2ST::NoSessions,
            size: resp.len() as u32 + 2,
            rc: 0,
        }
        .try_marshal(resp)?;
        Ok(())
    });
    let cmd = TestCommand(2);
    assert_eq!(
        client.run_command(&cmd),
        Err(TssTcsError::new(TssErrorCode::OutOfMemory).into())
    );
}

// TODO: We can probably do this better; for now just persist the current test.
pub struct FakeTpmConnection {
    len: usize,
    response: [u8; RESP_BUFFER_SIZE],
    header: RespHeader,
}
impl Default for FakeTpmConnection {
    fn default() -> Self {
        FakeTpmConnection {
            len: 0,
            response: [0; RESP_BUFFER_SIZE],
            header: RespHeader {
                tag: TPM2ST::NoSessions,
                size: 0,
                rc: 0,
            },
        }
    }
}
impl TpmConnection for FakeTpmConnection {
    fn transact(&mut self, _: &[u8], response: &mut [u8]) -> TssResult<()> {
        let off = self.header.try_marshal(response)?;
        let length = off + self.len;
        if self.len > response.len() {
            return Err(TPM_RC_SIZE.into());
        }
        response[off..length].copy_from_slice(&self.response[..self.len]);
        self.header.size = length as u32;
        self.header.try_marshal(response)?;
        Ok(())
    }
}
impl FakeTpmConnection {
    fn add_to_response<M: Marshalable>(&mut self, val: &M) {
        self.len += val.try_marshal(&mut self.response[self.len..]).unwrap()
    }
}

#[derive(Marshalable)]
struct TestHandlesCommand();
impl TpmCommand for TestHandlesCommand {
    const CMD_CODE: TPM2CC = TPM2CC::NVUndefineSpaceSpecial;
    type Handles = TPM2Handle;
    type RespT = ();
    type RespHandles = TPM2Handle;
}

#[test]
fn test_response_missing_handles() {
    let cmd = TestHandlesCommand();
    let mut client = TpmClient::new(FakeTpmConnection::default());
    assert_eq!(client.run_command(&cmd), Err(TPM_RC_MEMORY.into()));
}

#[test]
fn test_response_missing_sessions() {
    let mut fake_connection = FakeTpmConnection::default();
    // Respond with the single response handle.
    fake_connection.add_to_response(&TPM2Handle(77));

    let cmd = TestHandlesCommand();
    let mut sessions = CmdSessions::default();
    let mut session = PasswordSession::default();
    sessions.push(&mut session);
    let mut client = TpmClient::new(fake_connection);
    assert_eq!(
        client.run_command_with_handles(&cmd, TPM2Handle::RSPW, sessions),
        Err(TPM_RC_MEMORY.into())
    );
}

#[test]
fn test_response_session_fails_validation() {
    let mut fake_connection = FakeTpmConnection::default();
    // Respond with the single response handle, and an invalid password auth.
    fake_connection.add_to_response(&TPM2Handle(77));
    let mut invalid_auth = TpmsAuthResponse::default();
    invalid_auth.session_attributes = TpmaSession(0xf);
    let validation_failure = PasswordSession::default().validate_auth_response(&invalid_auth);
    assert!(validation_failure.is_err());
    fake_connection.add_to_response(&invalid_auth);

    let cmd = TestHandlesCommand();
    let mut sessions = CmdSessions::default();
    let mut session = PasswordSession::default();
    sessions.push(&mut session);
    let mut client = TpmClient::new(fake_connection);
    assert_eq!(
        client.run_command_with_handles(&cmd, TPM2Handle::RSPW, sessions),
        Err(validation_failure.err().unwrap())
    );
}
