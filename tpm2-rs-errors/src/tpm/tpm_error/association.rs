// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
mod handle;
pub use handle::*;
mod parameter;
pub use parameter::*;
mod session;
pub use session::*;

// =============================================================================
// CONSTANTS
// =============================================================================

/// Add to a parameter-related error.
///
/// Purposefully private to this module.
const TPM_RC_P: u32 = 0x040;

/// Add to a session-related error.
///
/// Purposefully private to this module.
const TPM_RC_S: u32 = 0x800;

// =============================================================================
// TYPES
// =============================================================================

#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
pub enum TpmErrorAssociation {
  Parameter(TpmErrorParameter),
  Handle(TpmErrorHandle),
  Session(TpmErrorSession),
}

// =============================================================================
// IMPLEMENTATION
// =============================================================================

impl TpmErrorAssociation {
  pub(super) fn from_format1_code(code: u32) -> Option<Self> {
    if code & TPM_RC_P != 0 {
      TpmErrorParameter::from_format1_code(code).map(|a| Self::Parameter(a))
    } else if code & TPM_RC_S != 0 {
      TpmErrorSession::from_format1_code(code).map(|a| Self::Session(a))
    } else {
      TpmErrorHandle::from_format1_code(code).map(|a| Self::Handle(a))
    }
  }
}
