// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;

// =============================================================================
// TYPE
// =============================================================================

/// TpmaSession defines the attributes of a session (TPMA_SESSION).
/// See definition in Part 2: Structures, section 8.4.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug, Default, Marshalable)]
pub struct TpmaSession(pub u8);

// =============================================================================
// IMPLEMENTATION
// =============================================================================

// -----------------------------------------------------------------------------
bitflags! {
    impl TpmaSession : u8 {
        /// Indicates if the session is to remain active (in commands) or does remain active (in reponses) after successful completion of the command.
        const CONTINUE_SESSION = 1 << 0;
        /// Indicates if the command should only be executed if the session is exclusive at the start of the command (in commands) or is exclusive (in responses).
        const AUDIT_EXCLUSIVE = 1 << 1;
        /// Indicates if the audit digest of the session should be initialized and exclusive status set in commands.
        const AUDIT_RESET = 1 << 2;
        /// Indicates if the first parameter in the command is symmetrically encrpyted.
        const DECRYPT = 1 << 5;
        /// Indicates if the session should (in commands) or did (in responses) encrypt the first parameter in the response.
        const ENCRYPT = 1 << 6;
        /// Indicates that the session is for audit, and that AUDIT_EXLCUSIVE/AUDIT_RESET have meaning.
        const AUDIT = 1 << 7;
    }
}
