// =============================================================================
// COMMON PRELUDE
//   Anything external that you want to be available everywhere should be added
//   here. Please keep this list sorted alphabetically, and try to avoid adding
//   using statements with * to show what we are actually depending on.
// =============================================================================

// -----------------------------------------------------------------------------
pub use bitflags::bitflags;
pub use core::cmp::min;
pub use core::mem::size_of;
pub use open_enum::open_enum;
pub use tpm2_rs_errors::TPM_RC_MEMORY;
pub use tpm2_rs_errors::TPM_RC_SIZE;
pub use tpm2_rs_errors::TPM_RC_VALUE;
pub use tpm2_rs_errors::TpmError;
pub use tpm2_rs_errors::TpmResult;
pub use tpm2_rs_marshal::Marshalable;
pub use tpm2_rs_marshal::MarshalableEnum;
pub use tpm2_rs_marshal::UnmarshalBuf;
pub use tpm2_rs_unionify::UnionSize;
