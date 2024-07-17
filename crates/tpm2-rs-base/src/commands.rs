// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
mod get_capability;
pub use get_capability::*;
mod pcr_read;
pub use pcr_read::*;
mod startup;
pub use startup::*;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;

// =============================================================================
// COMMAND
// =============================================================================

/// Trait for a TPM command transaction.
pub trait TpmCommand: Marshalable {
    /// The command code.
    const CMD_CODE: TPM2CC;
    /// The command handles type.
    type Handles: Marshalable + Default;
    /// The response parameters type.
    type RespT: Marshalable;
    /// The reponse handles type.
    type RespHandles: Marshalable;
}
