// =============================================================================
// ATTRIBUTES
// =============================================================================

#![cfg_attr(not(test), no_std)]
#![allow(dead_code, clippy::large_enum_variant)]
#![forbid(unsafe_code)]

// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
// PRELUDE MODULE
//   Never mark any of these modules as `pub`, just for common crate-only uses.
// -----------------------------------------------------------------------------
mod prelude;
pub(crate) use prelude::*;

// -----------------------------------------------------------------------------
// PUBLIC MODULES
//   These should always be marked `pub`, they will always be exposed.
// -----------------------------------------------------------------------------

// Justification:
//   Commands are special structures based on the command definition in the
//   spec. Separation between the fundamental types and command types is thus,
//   somewhatly useful.
pub mod commands;

// -----------------------------------------------------------------------------
// RE-EXPORTED MODULES
//   Whether or not a module is pub or re-export just depends on how we want the
//   client to construct the `use` statement. In this case, the constants and
//   specification types aren't interesting to namespace.
// -----------------------------------------------------------------------------
mod constants;
pub use constants::*;
mod types;
pub use types::*;

// -----------------------------------------------------------------------------
// TESTS
//   TODO: Move these closer to the tested code instead of having one test mod.
//   We should do less of these huge test files with all the tests in one file.
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests;
