// =============================================================================
// COMMON USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;

// =============================================================================
// COMMON FUNCTIONS FOR SUBMODULES
// =============================================================================

/// Returns an attribute field built by applying the mask/shift to the value.
const fn new_attribute_field(value: u32, mask: u32, shift: u32) -> u32 {
  (value << shift) & mask
}

/// Returns the attribute field retrieved from the value with the mask/shift.
const fn get_attribute_field(value: u32, mask: u32, shift: u32) -> u32 {
  (value & mask) >> shift
}

/// Sets the attribute field defined by mask/shift in the value to the field value, and returns the result.
const fn set_attribute_field(value: u32, field_value: u32, mask: u32, shift: u32) -> u32 {
  (value & !mask) | new_attribute_field(field_value, mask, shift)
}

// =============================================================================
// TYPES
// =============================================================================

/// TpmaAlgorithm defines the attributes of an algorithm (TPMA_ALGORITHM).
/// See definition in Part 2: Structures, section 8.2.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug, Default, Marshalable)]
pub struct TpmaAlgorithm(pub u32);
bitflags! {
    impl TpmaAlgorithm : u32 {
        /// Indicates an asymmetric algorithm with public and private portions.
        const ASYMMETRIC = 1 << 0;
        /// Indicates a symmetric block cipher.
        const SYMMETRIC = 1 << 1;
        /// Indicates a hash algorithm.
        const HASH = 1 << 2;
        /// Indicates an algorithm that may be used as an object type.
        const OBJECT = 1 << 3;
        /// Indicates a signing algorithm.
        const SIGNING = 1 << 8;
        /// Indicates an encryption/decryption algorithm.
        const ENCRYPTING = 1 << 9;
        /// Indicates a method such as a key derivative function.
        const METHOD = 1 << 10;
    }
}

/// TpmaCc defines the attributes of a command (TPMA_CC).
/// See definition in Part 2: Structures, section 8.9.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug, Default, Marshalable)]
pub struct TpmaCc(pub u32);
bitflags! {
    impl TpmaCc : u32 {
        /// Whether the command may write to NV.
        const NV  = 1 << 22;
        /// Whether the command could flush any number of loaded contexts.
        const EXTENSIVE = 1 << 23;
        /// Whether the conext associated with any transient handle in the command will be flushed when this command completes.
        const FLUSHED = 1 << 24;
        /// Wether there is a handle area in the response.
        const R_HANDLE = 1 << 28;
        /// Whether the command is vendor-specific.
        const V = 1 << 29;
        // See multi-bit fields below.
        const _ = !0;
    }
}
impl TpmaCc {
    /// Shift for the command index field.
    const COMMAND_INDEX_SHIFT: u32 = 0;
    /// Mask for the command index field.
    const COMMAND_INDEX_MASK: u32 = 0xFFFF;
    /// Shift for the command handles field.
    const C_HANDLES_SHIFT: u32 = 25;
    /// Mask for the command handles field.
    const C_HANDLES_MASK: u32 = 0x7 << TpmaCc::C_HANDLES_SHIFT;

    /// Creates a TpmaCc with the command index field set to the provided value.
    pub(crate) const fn command_index(index: u16) -> TpmaCc {
        TpmaCc(new_attribute_field(
            index as u32,
            Self::COMMAND_INDEX_MASK,
            Self::COMMAND_INDEX_SHIFT,
        ))
    }
    /// Creates a TpmaCc with the command handles field set to the provided value.
    const fn c_handles(count: u32) -> TpmaCc {
        TpmaCc(new_attribute_field(
            count,
            Self::C_HANDLES_MASK,
            Self::C_HANDLES_SHIFT,
        ))
    }

    /// Returns the command being selected.
    pub(crate) fn get_command_index(&self) -> u16 {
        get_attribute_field(self.0, Self::COMMAND_INDEX_MASK, Self::COMMAND_INDEX_SHIFT) as u16
    }
    /// Returns the number of handles in the handle area for this command.
    pub(crate) fn get_c_handles(&self) -> u32 {
        get_attribute_field(self.0, Self::C_HANDLES_MASK, Self::C_HANDLES_SHIFT)
    }

    /// Sets the command being selected.
    pub(crate) fn set_command_index(&mut self, index: u16) {
        self.0 = set_attribute_field(
            self.0,
            index as u32,
            Self::COMMAND_INDEX_MASK,
            Self::COMMAND_INDEX_SHIFT,
        );
    }
    /// Sets the number of handles in the handle area for this command.
    pub(crate) fn set_c_handles(&mut self, count: u32) {
        self.0 = set_attribute_field(self.0, count, Self::C_HANDLES_MASK, Self::C_HANDLES_SHIFT);
    }
}

/// TpmaLocality represents the locality attribute (TPMA_LOCALITY).
/// See definition in Part 2: Structures, section 8.5.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug, Default, Marshalable)]
pub struct TpmaLocality(pub u8);
bitflags! {
    impl TpmaLocality : u8 {
        const LOC_ZERO = 1 << 0;
        const LOC_ONE = 1 << 1;
        const LOC_TWO = 1 << 2;
        const LOC_THREE = 1 << 3;
        const LOC_FOUR = 1 << 4;
        // If any other bits are set, an extended locality is indicated.
        const _ = !0;
    }
}
impl TpmaLocality {
    const EXTENDED_LOCALITY_MASK: u8 = 0xE0;
    /// Returns whether this attribute indicates an extended locality.
    fn is_extended(&self) -> bool {
        (self.0 & Self::EXTENDED_LOCALITY_MASK) != 0
    }
}
/// TpmaObject indicates an object's use, authorization types, and relationship to other objects (TPMA_OBJECT).
/// See definition in Part 2: Structures, section 8.3.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug, Default, Marshalable)]
pub struct TpmaObject(pub u32);
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

/// TpmNv represents the NV index attributes (TPMA_NV).
/// See definition in Part 2: Structures, section 13.4.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug, Default, Marshalable)]
pub struct TpmaNv(pub u32);
bitflags! {
  impl TpmaNv : u32 {
      /// Whether the index data can be written if platform authorization is provided.
      const PPWRITE = 1 << 0;
      /// Whether the index data can be written if owner authorization is provided.
      const OWNERWRITE = 1 <<  1;
      /// Whether authorizations to change the index contents that require USER role may be provided with an HMAC session or password.
      const AUTHWRITE = 1 << 2;
      /// Whether authorizations to change the index contents that require USER role may be provided with a policy session.
      const POLICYWRITE = 1 << 3;
      /// If set, the index may not be deteled unless the auth_policy is satisfied using nv_undefined_space_special.
      /// If clear, the index may be deleted with proper platform/owner authorization using nv_undefine_space.
      const POLICY_DELETE = 1 << 10;
      /// Whether the index can NOT be written.
      const WRITELOCKED = 1 << 11;
      /// Whether a partial write of the index data is NOT allowed.
      const WRITEALL = 1 << 12;
      /// Whether nv_write_lock may be used to prevent futher writes to this location.
      const WRITEDEFINE = 1 << 13;
      /// Whether nv_write_lock may be used to prevent further writes to this location until the next TPM reset/restart.
      const WRITE_STCLEAR = 1 << 14;
      /// Whether WRITELOCKED is set if nv_global_write_lock is successful.
      const GLOBALLOCK = 1 << 15;
      /// Whether the index data can be read if platform authorization is provided.
      const PPREAD = 1 << 16;
      /// Whether the index data can be read if owner authorization is provided.
      const OWNERREAD = 1 << 17;
      /// Whether the index data can be read if auth_value is provided.
      const AUTHREAD = 1 << 18;
      /// Whether the index data can be read if the auth_policy is satisfied.
      const POLICYREAD = 1 << 19;
      /// If set, authorizationn failures of the index do not affect the DA logic and authorization of the index is not blocked when the TPM is in Lockout mode.
      /// If clear, authorization failures of the index will increment the authorization failure counter and authorizations of this index are not allowed when the TPM is in Lockout mode.
      const NO_DA = 1 << 25;
      /// Whether NV index state is required to be saved only when the TPM performs an orderly shutdown.
      const ORDERLY = 1 << 26;
      /// Whether WRITTEN is cleared by TPM reset/restart.
      const CLEAR_STCLEAR = 1 << 27;
      /// Whether reads of the index are blocked  until the next TPM reset/restart.
      const READLOCKED = 1 << 28;
      /// Whether the index has been written.
      const WRITTEN = 1 << 29;
      /// If set, the index may be undefined with platform authorization but not owner authorization.
      /// If clear, the index may be undefined with owner authorization but not platform authorization.
      const PLATFORMCREATE = 1 << 30;
      /// Whether nv_read_lock may be used to set READLOCKED for this index.
      const READ_STCLEAR = 1 << 31;
      // See multi-bit type field below.
      const _ = !0;
  }
}
impl TpmaNv {
  /// Mask for the index type field.
  const NT_MASK: u32 = 0xF0;
  /// Shift of the index type field.
  const NT_SHIFT: u32 = 4;

  /// Returns the attribute for an index type (with all other field clear).
  const fn from_index_type(index_type: TPM2NT) -> TpmaNv {
      TpmaNv(new_attribute_field(
          index_type.0 as u32,
          Self::NT_MASK,
          Self::NT_SHIFT,
      ))
  }

  /// Returns the type of the index.
  pub fn get_index_type(&self) -> TPM2NT {
      TPM2NT(get_attribute_field(self.0, Self::NT_MASK, Self::NT_SHIFT) as u8)
  }
  /// Sets the type of the index.
  pub fn set_type(&mut self, index_type: TPM2NT) {
      self.0 = set_attribute_field(self.0, index_type.0 as u32, Self::NT_MASK, Self::NT_SHIFT);
  }
}
impl From<TPM2NT> for TpmaNv {
  fn from(value: TPM2NT) -> Self {
      Self::from_index_type(value)
  }
}

/// TpmaSession defines the attributes of a session (TPMA_SESSION).
/// See definition in Part 2: Structures, section 8.4.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug, Default, Marshalable)]
pub struct TpmaSession(pub u8);
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
