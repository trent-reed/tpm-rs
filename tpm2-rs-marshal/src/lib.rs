// =============================================================================
// ATTRIBUTES
// =============================================================================

#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]

// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
// PRIVATE MODULES
//   Never mark any of these modules as `pub`, we don't want to expose them.
// -----------------------------------------------------------------------------
mod prelude;
pub(crate) use prelude::*;  // Include the prelude for the crate.

// Provide a re-export of the marshal crate within itself for proper derives.
extern crate self as tpm2_rs_marshal;

// -----------------------------------------------------------------------------
// DERIVE DEPENDENCIES
//   We need to expose these from the crate for the derive macro, but we don't
//   want folks to take a dependency on it. Just mark it as hidden.
// -----------------------------------------------------------------------------
#[doc(hidden)]
pub mod __private {
  pub use tpm2_rs_errors::TssErrorCode;
  pub use tpm2_rs_errors::TssTspError;
  pub use tpm2_rs_errors::TssTspResult;
}

// -----------------------------------------------------------------------------
// RE-EXPORTED MODULES
//   Never mark any of these modules as `pub`, we don't want to expose them.
//   We will expose the subtypes of the module though, just not the module.
// -----------------------------------------------------------------------------
pub use tpm2_rs_marshal_derive::Marshalable;
mod traits;
pub use traits::*;
mod types;
pub use types::*;

// -----------------------------------------------------------------------------
// TESTS
//   TODO: Move these closer to the tested code instead of having one test mod.
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests;
