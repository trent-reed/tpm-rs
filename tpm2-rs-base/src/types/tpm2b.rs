// =============================================================================
// MACROS
//   These need to come first based on how macro rules work. You must define the
//   macro before the usage in the module file.
//
//   TODO: We can probably get rid of these with some helper subtypes.
// =============================================================================

// -----------------------------------------------------------------------------
// TODO: I highly suspect this can go away... But I need to keep refactoring.
macro_rules! impl_try_marshalable_tpm2b_simple {
  ($T:ty, $F:ident) => {
      impl crate::types::Tpm2bSimple for $T {
          const MAX_BUFFER_SIZE: usize = size_of::<$T>() - size_of::<u16>();

          // TODO: Used in Marshalable, but is this all really necessary?
          //       Also used in client a little.
          fn get_size(&self) -> u16 {
              self.size
          }

          // TODO: I have no idea what this function is for...
          fn get_buffer(&self) -> &[u8] {
              &self.$F[0..self.get_size() as usize]
          }

          // TODO: I have no idea what this function is for...
          //       I assume it's to check bounds on buffers or something,
          //       It's only used in tests and the tests don't make it clearer.
          //       Maybe old code that used to be used in marshaling and can be
          //       removed now?
          fn from_bytes(buffer: &[u8]) -> TssTspResult<Self> {
              // Overflow check
              if buffer.len() > core::cmp::min(u16::MAX as usize, Self::MAX_BUFFER_SIZE) {
                  return Err(TssTspError::new(TssErrorCode::InternalError));
              }

              let mut dest: Self = Self {
                  size: buffer.len() as u16,
                  $F: [0; Self::MAX_BUFFER_SIZE],
              };
              dest.$F[..buffer.len()].copy_from_slice(buffer);
              Ok(dest)
          }
      }

      impl Default for $T {
          fn default() -> Self {
              use crate::types::Tpm2bSimple;
              Self {
                  size: 0,
                  $F: [0; Self::MAX_BUFFER_SIZE],
              }
          }
      }

      impl Marshalable for $T {
          fn try_unmarshal(buffer: &mut UnmarshalBuf) -> TssTspResult<Self> {
            use crate::types::Tpm2bSimple;
              let got_size = u16::try_unmarshal(buffer)?;
              // Ensure the buffer is large enough to fullfill the size indicated
              // TODO: Probably we need to clean up UnmarshalBuf.
              let sized_buffer = buffer.get(got_size as usize);
              if !sized_buffer.is_some() {
                  return Err(TssTspError::new(TssErrorCode::InternalError));
              }

              let mut dest: Self = Self {
                  size: got_size,
                  $F: [0; Self::MAX_BUFFER_SIZE],
              };

              // Make sure the size indicated isn't too large for the types buffer
              if sized_buffer.unwrap().len() > dest.$F.len() {
                  return Err(TssTspError::new(TssErrorCode::InternalError));
              }
              dest.$F[..got_size.into()].copy_from_slice(&sized_buffer.unwrap());

              Ok(dest)
          }

          fn try_marshal(&self, buffer: &mut [u8]) -> TssTspResult<usize> {
              use crate::types::Tpm2bSimple;
              let used = self.size.try_marshal(buffer)?;
              let (_, rest) = buffer.split_at_mut(used);
              let buffer_marsh = self.get_size() as usize;
              if buffer_marsh > (core::cmp::max(Self::MAX_BUFFER_SIZE, rest.len())) {
                  return Err(TssTspError::new(TssErrorCode::InternalError));
              }
              rest[..buffer_marsh].copy_from_slice(&self.$F[..buffer_marsh]);
              Ok(used + buffer_marsh)
          }
      }
  };
}

// -----------------------------------------------------------------------------
macro_rules! impl_try_marshalable_tpm2b_struct {
  ($T:ty, $StructType:ty, $F:ident) => {
      impl crate::types::Tpm2bStruct for $T {
          type StructType = $StructType;

          fn from_struct(val: &Self::StructType) -> TssTspResult<Self> {
              let mut x = Self::default();
              x.size = val.try_marshal(&mut x.$F)? as u16;
              Ok(x)
          }

          fn to_struct(&self) -> TssTspResult<Self::StructType> {
              use crate::types::Tpm2bSimple;
              let mut buf = UnmarshalBuf::new(&self.$F[0..self.get_size() as usize]);
              Self::StructType::try_unmarshal(&mut buf)
          }
      }
  };
}

// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
mod tpm2b_attest;
pub use tpm2b_attest::*;
mod tpm2b_auth;
pub use tpm2b_auth::*;
mod tpm2b_context_data;
pub use tpm2b_context_data::*;
mod tpm2b_context_sensitive;
pub use tpm2b_context_sensitive::*;
mod tpm2b_creation_data;
pub use tpm2b_creation_data::*;
mod tpm2b_data;
pub use tpm2b_data::*;
mod tpm2b_derive;
pub use tpm2b_derive::*;
mod tpm2b_digest;
pub use tpm2b_digest::*;
mod tpm2b_ecc_parameter;
pub use tpm2b_ecc_parameter::*;
mod tpm2b_ecc_point;
pub use tpm2b_ecc_point::*;
mod tpm2b_encrypted_secret;
pub use tpm2b_encrypted_secret::*;
mod tpm2b_event;
pub use tpm2b_event::*;
mod tpm2b_id_object;
pub use tpm2b_id_object::*;
mod tpm2b_iv;
pub use tpm2b_iv::*;
mod tpm2b_label;
pub use tpm2b_label::*;
mod tpm2b_max_buffer;
pub use tpm2b_max_buffer::*;
mod tpm2b_max_cap_buffer;
pub use tpm2b_max_cap_buffer::*;
mod tpm2b_max_nv_buffer;
pub use tpm2b_max_nv_buffer::*;
mod tpm2b_name;
pub use tpm2b_name::*;
mod tpm2b_nonce;
pub use tpm2b_nonce::*;
mod tpm2b_nv_public;
pub use tpm2b_nv_public::*;
mod tpm2b_operand;
pub use tpm2b_operand::*;
mod tpm2b_private_key_rsa;
pub use tpm2b_private_key_rsa::*;
mod tpm2b_private_vendor_specific;
pub use tpm2b_private_vendor_specific::*;
mod tpm2b_private;
pub use tpm2b_private::*;
mod tpm2b_public_key_rsa;
pub use tpm2b_public_key_rsa::*;
mod tpm2b_public;
pub use tpm2b_public::*;
mod tpm2b_sensitive_create;
pub use tpm2b_sensitive_create::*;
mod tpm2b_sensitive_data;
pub use tpm2b_sensitive_data::*;
mod tpm2b_sensitive;
pub use tpm2b_sensitive::*;
mod tpm2b_simple;
pub use tpm2b_simple::*;
mod tpm2b_struct;
pub use tpm2b_struct::*;
mod tpm2b_sym_key;
pub use tpm2b_sym_key::*;
mod tpm2b_template;
pub use tpm2b_template::*;

// =============================================================================
// COMMON USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;
