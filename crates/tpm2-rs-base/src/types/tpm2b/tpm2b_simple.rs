use super::*;

// TODO: Non-spec types like this should have a different name.
// TODO: These traits may belong in a different module for clarity.
// Helper for splitting up ranges of an unmarshal buffer.
pub trait Tpm2bSimple {
  const MAX_BUFFER_SIZE: usize;
  fn get_size(&self) -> u16;
  fn get_buffer(&self) -> &[u8];
  fn from_bytes(buffer: &[u8]) -> TpmResult<Self>
  where
      Self: Sized;
}
