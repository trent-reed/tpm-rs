// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;

// The Marshalable trait defines the API for {un}marshaling TPM structs. It
// is implemented for primitive types. The marshal_derive::Marshal macro
// will provide an implementation that calls try_{un}marshal for each of
// it's fields, but beware that this will not produce the correct output
// for types that have variable sized marshaling output based on their
// contents. Types can also provide their own implementation if needed.
//
// Enums with fields require a primitive representation and explicit selector
// values. These will be marshaled as the primitive selector followed by the
// selected variant.
//
// Array fields which should only {un}marshal a subset of their entries can
// use the length attribute to specify the field providing the number of
// entries that should be marshaled. See TpmlPcrSelection for an example.
pub trait Marshalable: Sized {
  // Unmarshals self from the prefix of `buffer`. Returns the unmarshalled self and number of bytes used.
  fn try_unmarshal(buffer: &mut UnmarshalBuf) -> TssTspResult<Self>;

  // Marshals self into the prefix of `buffer`. Returns the number of bytes used.
  fn try_marshal(&self, buffer: &mut [u8]) -> TssTspResult<usize>;
}

impl Marshalable for () {
  fn try_marshal(&self, _buffer: &mut [u8]) -> TssTspResult<usize> {
      Ok(0)
  }
  fn try_unmarshal(_buffer: &mut UnmarshalBuf) -> TssTspResult<Self> {
      Ok(())
  }
}

impl<const M: usize> Marshalable for [u8; M] {
  fn try_unmarshal(buffer: &mut UnmarshalBuf) -> TssTspResult<Self> {
      if let Some(mine) = buffer.get(M) {
          let mut x = [0u8; M];
          x.copy_from_slice(mine);
          Ok(x)
      } else {
        return Err(TssTspError::new(TssErrorCode::InternalError));
      }
  }

  fn try_marshal(&self, buffer: &mut [u8]) -> TssTspResult<usize> {
      if buffer.len() >= self.len() {
          buffer[..self.len()].copy_from_slice(self);
          Ok(self.len())
      } else {
        return Err(TssTspError::new(TssErrorCode::InternalError));
      }
  }
}

// Helper to define Marshalable for primitive types with {to,from}_be_bytes methods.
// T is the primitive type.
macro_rules! impl_be_prim_marshalable {
  ($T:ty) => {
      impl Marshalable for $T {
          fn try_unmarshal(buffer: &mut UnmarshalBuf) -> TssTspResult<Self> {
              type BufferType = [u8; size_of::<$T>()];
              let x = BufferType::try_unmarshal(buffer)?;
              // let x = <[u8; size_of::<$T>()]>::try_unmarshal(buffer)?;
              Ok(Self::from_be_bytes(x))
          }

          fn try_marshal(&self, buffer: &mut [u8]) -> TssTspResult<usize> {
              self.to_be_bytes().try_marshal(buffer)
          }
      }
  };
}
impl_be_prim_marshalable! {u8}
impl_be_prim_marshalable! {u16}
impl_be_prim_marshalable! {u32}
impl_be_prim_marshalable! {u64}
impl_be_prim_marshalable! {i8}
impl_be_prim_marshalable! {i16}
impl_be_prim_marshalable! {i32}
impl_be_prim_marshalable! {i64}
