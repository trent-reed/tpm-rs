// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
mod association;
pub use association::*;
mod format0;
pub use format0::*;
mod format1;
pub use format1::*;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;

// =============================================================================
// CONSTANTS
// =============================================================================

/// This bit is SET in all format 1 response codes. The codes in this group may
/// have a value added to them to indicate the handle, session, or parameter to
/// which they apply.
///
/// Purposefully private to this module.
const RC_FMT1: u32 = 0x080;

// =============================================================================
// TYPES
// =============================================================================

#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
pub enum TpmErrorFormat {
  Zero,
  One,
}

#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
pub enum TpmErrorOrigin {
  Tcg,
  Vendor,
}

#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
pub enum TpmErrorSeverity {
  Error,
  Warning,
}

#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
pub enum TpmErrorVersion {
  Tpm1_2,
  Tpm2,
}

// Technically the TPM only uses the first 12 bits.
// The remaining bits are used by software.
#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
#[repr(transparent)]
pub struct TpmError(NonZeroU32);

// =============================================================================
// IMPLEMENTATION
// =============================================================================

// -----------------------------------------------------------------------------
impl TpmError {
  pub const fn data(&self) -> u32 {
    self.0.get() >> 12
  }
  pub const fn as_u32(&self) -> u32 {
    self.0.get()
  }
  pub const fn format(&self) -> TpmErrorFormat {
    if self.0.get() & RC_FMT1 == 0 {
      TpmErrorFormat::Zero
    } else {
      TpmErrorFormat::One
    }
  }
  pub const fn version(&self) -> TpmErrorVersion {
    match self.format() {
      TpmErrorFormat::Zero =>
        TpmFmt0Error::new_unchecked(*self).version(),
      TpmErrorFormat::One =>
        TpmFmt1Error::new_unchecked(*self).version(),
    }
  }
  pub const fn as_format0(&self) -> Option<TpmFmt0Error> {
    match self.format() {
      TpmErrorFormat::Zero => Some(TpmFmt0Error::new_unchecked(*self)),
      TpmErrorFormat::One => None,
    }
  }
  pub const fn as_format1(&self) -> Option<TpmFmt1Error> {
    match self.format() {
      TpmErrorFormat::Zero => None,
      TpmErrorFormat::One => Some(TpmFmt1Error::new_unchecked(*self)),
    }
  }
  pub const fn code(&self) -> u8 {
    match self.format() {
      TpmErrorFormat::Zero =>
        TpmFmt0Error::new_unchecked(*self).code(),
      TpmErrorFormat::One =>
        TpmFmt1Error::new_unchecked(*self).code(),
    }
  }
  pub fn association(&self) -> Option<TpmErrorAssociation> {
    self.as_format1().as_ref().and_then(TpmFmt1Error::association)
  }
  pub const fn new(value: u32) -> Option<Self> {
    match NonZeroU32::new(value) {
      Some(value) => Some(Self(value)),
      None => None,
    }
  }
  // Panics!
  pub const fn new_checked(value: u32) -> Self {
    match Self::new(value) {
      Some(value) => value,
      None => panic!("TpmRcError cannot be 0"),
    }
  }
}

// =============================================================================
// TRAITS
// =============================================================================

// -----------------------------------------------------------------------------
impl From<NonZeroU32> for TpmError {
  fn from(value: NonZeroU32) -> Self {
    Self(value)
  }
}

// -----------------------------------------------------------------------------
impl From<TpmError> for NonZeroU32 {
  fn from(value: TpmError) -> Self {
    value.0
  }
}
