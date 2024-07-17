use super::*;

// TODO: Non-spec types like this should have a different name.
// TODO: These traits may belong in a different module for clarity.
/// Provides conversion to/from a struct type for TPM2B types that don't hold a bytes buffer.
pub trait Tpm2bStruct: Tpm2bSimple {
  type StructType: Marshalable;

  /// Marshals the value into the 2b holder.
  fn from_struct(val: &Self::StructType) -> TpmResult<Self>
  where
      Self: Sized;

  /// Extracts the struct value from the 2b holder.
  fn to_struct(&self) -> TpmResult<Self::StructType>;
}
