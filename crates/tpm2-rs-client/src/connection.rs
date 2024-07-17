use super::*;

// A low-level connection, this deals with I/O for some abstract command.
#[cfg_attr(test, mockall::automock)]
pub trait TpmConnection {
  // TODO: Can this be a more specific error layer?
  fn transact(&mut self, command: &[u8], response: &mut [u8]) -> TssResult<()>;
}
