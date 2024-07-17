// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;

// =============================================================================
// TYPE
// =============================================================================

/// TpmaObject indicates an object's use, authorization types, and relationship to other objects (TPMA_OBJECT).
/// See definition in Part 2: Structures, section 8.3.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug, Default, Marshalable)]
pub struct TpmaObject(pub u32);

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
bitflags! {
    impl TpmaObject : u32 {
        /// Whether the hierarchy of the object may NOT change.
        const FIXED_TPM = 1 << 1;
        /// Whether saved contexts of this object may NO be loaded after startup(CLEAR).
        const ST_CLEAR = 1 << 2;
        /// Whether the parent of the object may NOT change.
        const FIXED_PARENT = 1 << 4;
        /// Whether the TPM generated all of the sensitive data, other than auth_value, when the object was created.
        const SENSITIVE_DATA_ORIGIN = 1 << 5;
        /// Whether approval of USER role actions with the object may be with an HMAC session or password using the auth_value of the object or a policy session.
        const USER_WITH_AUTH = 1 << 6;
        /// Whether approval of ADMIN role actions with the object may ONLY be done with a policy session.
        const ADMIN_WITH_POLICY = 1 << 7;
        /// Whether the object is NOT subject to dictionary attack protections.
        const NO_DA = 1 << 10;
        /// Whether, if the object is duplicated, symmetric_alg and new_parent_handle shall not be null.
        const ENCRYPTED_DUPLICATION = 1 << 11;
        /// Whether key usage is restricated to manipulate structures of known format.
        const RESTRICTED = 1 << 16;
        /// Whether the private portion of the key may be used to decrypt.
        const DECRYPT = 1 << 17;
        /// Whether the private portion of the key may be used to encrypt (for symmetric cipher objects) or sign.
        const SIGN_ENCRYPT = 1 << 18;
        /// Whether this is an asymmetric key that may not be used to sign with sign().
        const X509_SIGN = 1 << 19;
    }
}
