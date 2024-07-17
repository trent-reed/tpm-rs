// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
mod command_header;
pub use command_header::*;
mod response_header;
pub use response_header::*;

// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests;

// =============================================================================
// USE
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;
use tpm2_rs_base::commands::*; // This file will end up using all commands.
use tpm2_rs_base::TpmsAuthResponse;
use tpm2_rs_base::TPM2ST;

// =============================================================================
// CONSTANTS
// =============================================================================

// -----------------------------------------------------------------------------
pub const CMD_BUFFER_SIZE: usize = 4096;
pub const RESP_BUFFER_SIZE: usize = 4096;

// =============================================================================
// TYPES
// =============================================================================

// For now, we should just make this a struct without any associated trait. It's
// tempting to want to make the commands that we call into a trait, but I think
// we should instead make a comprehensive fake that injects itself as a
// "connection".
pub struct TpmClient<C> {
    connection: C,
}

// =============================================================================
// IMPLEMENTATION
// =============================================================================

/// Adds any command sessions to the command buffer.
fn write_command_sessions(sessions: &CmdSessions, buffer: &mut [u8]) -> TssResult<usize> {
    if sessions.is_empty() {
        return Ok(0);
    }
    let mut auth_offset = size_of::<u32>();
    for session in sessions {
        // TODO: Support parameter encryption.
        auth_offset += session
            .get_auth_command()
            .try_marshal(&mut buffer[auth_offset..])?;
    }
    let auth_size = (auth_offset - size_of::<u32>()) as u32;
    auth_size.try_marshal(buffer)?;
    Ok(auth_offset)
}

/// Umarshals the response header and checks the contained response code.
fn read_response_header(buffer: &[u8]) -> TssResult<(RespHeader, usize)> {
    let mut unmarsh = UnmarshalBuf::new(buffer);
    let resp_header = RespHeader::try_unmarshal(&mut unmarsh)?;
    // Very likely this is the only reasonable use of TPM error.
    if let Some(error) = TpmError::new(resp_header.rc) {
        return Err(error.into());
    }
    Ok((resp_header, buffer.len() - unmarsh.len()))
}

/// Unmarshals any response sessions.
fn read_response_sessions(sessions: &CmdSessions, buffer: &mut UnmarshalBuf) -> TssResult<()> {
    for session in sessions {
        // TODO: Support parameter decryption.
        let auth = TpmsAuthResponse::try_unmarshal(buffer)?;
        session.validate_auth_response(&auth)?;
    }
    Ok(())
}

// -----------------------------------------------------------------------------
impl<C: TpmConnection> TpmClient<C> {
    // Runs a command without handles.
    // TODO: Should this really be publicly visible?
    pub fn run_command<Command: TpmCommand>(&mut self, cmd: &Command) -> TssResult<Command::RespT> {
        Ok(self
            .run_command_with_handles(cmd, Command::Handles::default(), CmdSessions::default())?
            .0)
    }

    /// Runs a command with provided handles and sessions.
    fn run_command_with_handles<Command: TpmCommand>(
        &mut self,
        cmd: &Command,
        cmd_handles: Command::Handles,
        cmd_sessions: CmdSessions,
    ) -> TssResult<(Command::RespT, Command::RespHandles)> {
        let mut cmd_buffer = [0u8; CMD_BUFFER_SIZE];
        let mut cmd_header = CmdHeader::new(cmd_sessions.is_empty(), Command::CMD_CODE);
        let mut written = cmd_header.try_marshal(&mut cmd_buffer)?;

        written += cmd_handles.try_marshal(&mut cmd_buffer[written..])?;
        written += write_command_sessions(&cmd_sessions, &mut cmd_buffer[written..])?;
        written += cmd.try_marshal(&mut cmd_buffer[written..])?;

        // Update the command size
        cmd_header.size = written as u32;
        let _ = cmd_header.try_marshal(&mut cmd_buffer)?;

        let mut resp_buffer = [0u8; RESP_BUFFER_SIZE];
        self.connection
            .transact(&cmd_buffer[..written], &mut resp_buffer)?;

        use tpm2_rs_errors::{TssErrorCode, TssTcsError};
        let (resp_header, read) = read_response_header(&resp_buffer)?;
        let resp_size = resp_header.size as usize;
        if resp_size > resp_buffer.len() {
            return Err(TssTcsError::new(TssErrorCode::OutOfMemory).into());
        }
        let mut unmarsh = UnmarshalBuf::new(&resp_buffer[read..resp_size]);
        let resp_handles = Command::RespHandles::try_unmarshal(&mut unmarsh)?;
        if resp_header.tag == TPM2ST::Sessions {
            let _param_size = u32::try_unmarshal(&mut unmarsh)?;
        }
        let resp = Command::RespT::try_unmarshal(&mut unmarsh)?;
        read_response_sessions(&cmd_sessions, &mut unmarsh)?;

        if !unmarsh.is_empty() {
            return Err(TssTcsError::new(TssErrorCode::TpmUnexpected).into());
        }
        Ok((resp, resp_handles))
    }

    pub fn new(connection: C) -> Self {
      Self { connection }
    }

    pub fn get_capability(&mut self, command: &GetCapabilityCmd) -> TssResult<GetCapabilityResp> {
        self.run_command(command)
    }
}
