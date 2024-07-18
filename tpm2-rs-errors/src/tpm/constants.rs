// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;

// =============================================================================
// PRIVATE CONSTANTS
// =============================================================================

/// Set for warning response codes.
const RC_WARN: u32 = 0x900;

/// This bit is SET in all format 1 response codes. The codes in this group may
/// have a value added to them to indicate the handle, session, or parameter to
/// which they apply.
const RC_FMT1: u32 = 0x080;

/// Set for all format 0 response codes.
const RC_VER1: u32 = 0x100;

// =============================================================================
// PUBLIC CONSTANTS (TPM 1.2)
// =============================================================================

// -----------------------------------------------------------------------------
// FORMAT 0 RESPONSE CODES
//   These codes will never have an association (parameter, handle, or session)
//   mixed in. Thus they can be trivially represented by constants.
// -----------------------------------------------------------------------------

/// Defined for compatibility with TPM 1.2.
pub const TPM_RC_BAD_TAG: TpmFmt0Error = TpmFmt0Error::new_checked(0x01E);

/// TPM not initialized by TPM2_Startup or already initialized.
pub const TPM_RC_INITIALIZE: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x000);

/// commands not being accepted because of a TPM failure.
///
/// NOTE:
///   This can be returned by TPM2_GetTestResult() as the testResult parameter.
pub const TPM_RC_FAILURE: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x001);

/// Improper use of a sequence handle.
pub const TPM_RC_SEQUENCE: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x003);

/// Not currently used.
pub const TPM_RC_PRIVATE: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x00B);

/// Not currently used.
pub const TPM_RC_HMAC: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x019);

/// The command is disabled.
pub const TPM_RC_DISABLED: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x020);

/// command failed because audit sequence required exclusivity.
pub const TPM_RC_EXCLUSIVE: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x021);

/// Authorization handle is not correct for command.
pub const TPM_RC_AUTH_TYPE: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x024);

/// Command requires an authorization session for handle and it is not present.
pub const TPM_RC_AUTH_MISSING: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x025);

/// Policy failure in math operation or an invalid authPolicy value.
pub const TPM_RC_POLICY: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x026);

/// PCR check fail.
pub const TPM_RC_PCR: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x027);

/// PCR have changed since checked.
pub const TPM_RC_PCR_CHANGED: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x028);

/// For all commands other than TPM2_FieldUpgradeData(), this code indicates
/// that the TPM is in field upgrade mode; for TPM2_FieldUpgradeData(), this
/// code indicates that the TPM is not in field upgrade mode.
pub const TPM_RC_UPGRADE: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x02D);

/// Context ID counter is at maximum.
pub const TPM_RC_TOO_MANY_CONTEXTS: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x02E);

/// authValue or authPolicy is not available for selected entity.
pub const TPM_RC_AUTH_UNAVAILABLE: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x02F);

/// A _TPM_Init and Startup(CLEAR) is required before the TPM can resume
/// operation.
pub const TPM_RC_REBOOT: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x030);

/// The protection algorithms (hash and symmetric) are not reasonably balanced.
/// The digest size of the hash must be larger than the key size of the
/// symmetric algorithm.
pub const TPM_RC_UNBALANCED: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x031);

/// Command commandSize value is inconsistent with contents of the command
/// buffer; either the size is not the same as the octets loaded by the hardware
/// interface layer or the value is not large enough to hold a command header.
pub const TPM_RC_COMMAND_SIZE: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x042);

/// Command code not supported.
pub const TPM_RC_COMMAND_CODE: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x043);

/// The value of authorizationSize is out of range or the number of octets in
/// the Authorization Area is greater than required.
pub const TPM_RC_AUTHSIZE: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x044);

/// Use of an authorization session with a context command or another command
/// that cannot have an authorization session.
pub const TPM_RC_AUTH_CONTEXT: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x045);

/// NV offset + size is out of range.
pub const TPM_RC_NV_RANGE: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x046);

/// Requested allocation size is larger than allowed.
pub const TPM_RC_NV_SIZE: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x047);

/// NV access locked.
pub const TPM_RC_NV_LOCKED: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x048);

/// NV access authorization fails in command actions (this failure does not
/// affect lockout.action).
pub const TPM_RC_NV_AUTHORIZATION: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x049);

/// An NV Index is used before being initialized (written) or the state saved by
/// TPM2_Shutdown(STATE) could not be restored.
pub const TPM_RC_NV_UNINITIALIZED: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x04A);

/// Insufficient space for NV allocation.
pub const TPM_RC_NV_SPACE: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x04B);

/// NV Index or persistent object already defined.
pub const TPM_RC_NV_DEFINED: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x04C);

/// Context in TPM2_ContextLoad() is not valid.
pub const TPM_RC_BAD_CONTEXT: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x050);

/// cpHash value already set or not correct for use.
pub const TPM_RC_CPHASH: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x051);

/// Handle for parent is not a valid parent.
pub const TPM_RC_PARENT: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x052);

/// Some function needs testing.
pub const TPM_RC_NEEDS_TEST: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x053);

/// Returned when an internal function cannot process a request due to an
/// unspecified problem. This code is usually related to invalid parameters that
/// are not properly filtered by the input unmarshaling code.
pub const TPM_RC_NO_RESULT: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x054);

/// The sensitive area did not unmarshal correctly after decryption – this code
/// is used in lieu of the other unmarshaling errors so that an attacker cannot
/// determine where the unmarshaling error occurred.
pub const TPM_RC_SENSITIVE: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x055);

/// Largest version 1 code that is not a warning.
pub const RC_MAX_FM0: TpmFmt0Error = TpmFmt0Error::new_checked(RC_VER1 + 0x07F);

// =============================================================================
// PUBLIC CONSTANTS (TPM 2)
// =============================================================================

// -----------------------------------------------------------------------------
// FORMAT 1 RESPONSE CODES
//   These codes *MAY* have an association (parameter, handle, or session), so
//   they must be constructed using some helper functions. For now we'll just
//   leave them bare, which corresponds to having no association.
// -----------------------------------------------------------------------------

/// Asymmetric algorithm not supported or not correct.
pub const TPM_RC_ASYMMETRIC: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x001);

/// Inconsistent attributes.
pub const TPM_RC_ATTRIBUTES: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x002);

/// Hash algorithm not supported or not appropriate.
pub const TPM_RC_HASH: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x003);

/// Value is out of range or is not correct for the context.
pub const TPM_RC_VALUE: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x004);

/// Hierarchy is not enabled or is not correct for the use.
pub const TPM_RC_HIERARCHY: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x005);

/// Key size is not supported.
pub const TPM_RC_KEY_SIZE: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x007);

/// Mask generation function not supported.
pub const TPM_RC_MGF: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x008);

/// Mode of operation not supported.
pub const TPM_RC_MODE: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x009);

/// The type of the value is not appropriate for the use.
pub const TPM_RC_TYPE: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x00A);

/// The handle is not correct for the use.
pub const TPM_RC_HANDLE: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x00B);

/// Unsupported key derivation function or function not appropriate for use.
pub const TPM_RC_KDF: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x00C);

/// Value was out of allowed range.
pub const TPM_RC_RANGE: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x00D);

/// The authorization HMAC check failed and the DA counter was incremented, or
/// use of lockoutAuth is disabled.
pub const TPM_RC_AUTH_FAIL: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x00E);

/// Invalid nonce size or nonce value mismatch.
pub const TPM_RC_NONCE: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x00F);

/// Authorization requires assertion of PP.
pub const TPM_RC_PP: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x010);

/// Unsupported or incompatible scheme.
pub const TPM_RC_SCHEME: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x012);

/// Structure is the wrong size.
pub const TPM_RC_SIZE: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x015);

/// Unsupported symmetric algorithm or key size, or not appropriate for
/// instance.
pub const TPM_RC_SYMMETRIC: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x016);

/// Incorrect structure tag.
pub const TPM_RC_TAG: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x017);

/// Union selector is incorrect.
pub const TPM_RC_SELECTOR: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x018);

/// The TPM was unable to unmarshal a value because there were not enough octets
/// in the input buffer.
pub const TPM_RC_INSUFFICIENT: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x01A);

/// The signature is not valid.
pub const TPM_RC_SIGNATURE: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x01B);

/// Key fields are not compatible with the selected use.
pub const TPM_RC_KEY: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x01C);

/// A policy check failed.
pub const TPM_RC_POLICY_FAIL: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x01D);

/// Integrity check failed.
pub const TPM_RC_INTEGRITY: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x01F);

/// Invalid ticket.
pub const TPM_RC_TICKET: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x020);

/// Reserved bits not set to zero as required.
pub const TPM_RC_RESERVED_BITS: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x021);

/// Authorization failure without DA implications.
pub const TPM_RC_BAD_AUTH: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x022);

/// The policy has expired.
pub const TPM_RC_EXPIRED: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x023);

/// The commandCode in the policy is not the commandCode of the command or the
/// command code in a policy command references a command that is not
/// implemented.
pub const TPM_RC_POLICY_CC: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x024);

/// Public and sensitive portions of an object are not cryptographically bound.
pub const TPM_RC_BINDING: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x025);

/// Curve not supported.
pub const TPM_RC_CURVE: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x026);

/// Point is not on the required curve.
pub const TPM_RC_ECC_POINT: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x027);

/// The hierarchy is firmware-limited but the Firmware Secret is unavailable.
pub const TPM_RC_FW_LIMITED: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x028);

/// The hierarchy is SVN-limited but the Firmware SVN Secret associated with the
/// given SVN is unavailable.
pub const TPM_RC_SVN_LIMITED: TpmFmt1Error = TpmFmt1Error::new_checked(RC_FMT1 + 0x029);

// -----------------------------------------------------------------------------
// FORMAT 0 RESPONSE CODES (Warnings)
// -----------------------------------------------------------------------------

/// Gap for context ID is too large.
pub const TPM_RC_CONTEXT_GAP: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x001);

/// Out of memory for object contexts.
pub const TPM_RC_OBJECT_MEMORY: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x002);

/// Out of memory for session contexts.
pub const TPM_RC_SESSION_MEMORY: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x003);

/// Out of shared object/session memory or need space for internal operations.
pub const TPM_RC_MEMORY: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x004);

/// Out of session handles – a session must be flushed before a new session may
/// be created.
pub const TPM_RC_SESSION_HANDLES: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x005);

/// Out of object handles – the handle space for objects is depleted and a
/// reboot is required.
///
/// NOTE 1 This cannot occur on the reference implementation.
/// NOTE 2 There is no reason why an implementation would implement a design
/// that would deplete handle space. Platform specifications are encouraged to
/// forbid it.
pub const TPM_RC_OBJECT_HANDLES: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x006);

/// Bad locality.
pub const TPM_RC_LOCALITY: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x007);

/// the TPM has suspended operation on the command; forward progress was made
/// and the command may be retried (See TPM 2.0 Part 1, “Multi-tasking.”).
///
/// NOTE This cannot occur on the reference implementation.
pub const TPM_RC_YIELDED: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x008);

/// The command was canceled.
pub const TPM_RC_CANCELED: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x009);

/// TPM is performing self-tests.
pub const TPM_RC_TESTING: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x00A);

/// The 1st handle in the handle area references a transient object or session
/// that is not loaded.
pub const TPM_RC_REFERENCE_H0: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x010);

/// The 2nd handle in the handle area references a transient object or session
/// that is not loaded.
pub const TPM_RC_REFERENCE_H1: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x011);

/// The 3rd handle in the handle area references a transient object or session
/// that is not loaded.
pub const TPM_RC_REFERENCE_H2: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x012);

/// The 4th handle in the handle area references a transient object or session
/// that is not loaded.
pub const TPM_RC_REFERENCE_H3: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x013);

/// The 5th handle in the handle area references a transient object or session
/// that is not loaded.
pub const TPM_RC_REFERENCE_H4: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x014);

/// The 6th handle in the handle area references a transient object or session
/// that is not loaded.
pub const TPM_RC_REFERENCE_H5: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x015);

/// The 7th handle in the handle area references a transient object or session
/// that is not loaded.
pub const TPM_RC_REFERENCE_H6: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x016);

/// The 1st authorization session handle references a session that is not
/// loaded.
pub const TPM_RC_REFERENCE_S0: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x018);

/// The 2nd authorization session handle references a session that is not
/// loaded.
pub const TPM_RC_REFERENCE_S1: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x019);

/// The 3rd authorization session handle references a session that is not
/// loaded.
pub const TPM_RC_REFERENCE_S2: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x01A);

/// The 4th authorization session handle references a session that is not
/// loaded.
pub const TPM_RC_REFERENCE_S3: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x01B);

/// The 5th session handle references a session that is not loaded.
pub const TPM_RC_REFERENCE_S4: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x01C);

/// The 6th session handle references a session that is not loaded.
pub const TPM_RC_REFERENCE_S5: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x01D);

/// The 7th authorization session handle references a session that is not
/// loaded.
pub const TPM_RC_REFERENCE_S6: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x01E);

/// The TPM is rate-limiting accesses to prevent wearout of NV.
pub const TPM_RC_NV_RATE: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x020);

/// Authorizations for objects subject to DA protection are not allowed at this
/// time because the TPM is in DA lockout mode.
pub const TPM_RC_LOCKOUT: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x021);

/// The TPM was not able to start the command.
pub const TPM_RC_RETRY: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x022);

/// The command may require writing of NV and NV is not current accessible.
pub const TPM_RC_NV_UNAVAILABLE: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x023);

/// This value is reserved and shall not be returned by the TPM.
pub const TPM_RC_NOT_USED: TpmFmt0Error = TpmFmt0Error::new_checked(RC_WARN + 0x7F);
