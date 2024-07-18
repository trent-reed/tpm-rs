// =============================================================================
// USES
// =============================================================================

use std::ops::DerefMut;

// -----------------------------------------------------------------------------
use super::*;

// Format0 is association-less.
#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
#[repr(transparent)]
pub struct TpmFmt0Error(TpmError);

impl TpmFmt0Error {
  pub const fn data(&self) -> u32 {
    self.0.data()
  }
  pub const fn as_u32(&self) -> u32 {
    self.0.as_u32()
  }
  pub const fn code(&self) -> u8 {
    (self.as_u32() & 0b1111111) as u8
  }
  pub const fn format(&self) -> TpmErrorFormat {
    // We can only construct this type if it's for sure the right format.
    // As such, no need to check the actual data, just return the format.
    TpmErrorFormat::Zero
  }
  pub const fn version(&self) -> TpmErrorVersion {
    if self.as_u32() & (0b1<<8) == 0 {
      TpmErrorVersion::Tpm1_2
    } else {
      TpmErrorVersion::Tpm2
    }
  }
  pub const fn origin(&self) -> TpmErrorOrigin {
    if self.as_u32() & (0b1<<10) == 0 {
      TpmErrorOrigin::Tcg
    } else {
      TpmErrorOrigin::Vendor
    }
  }
  pub const fn severity(&self) -> TpmErrorSeverity {
    if self.as_u32() & (0b1<<11) == 0 {
      TpmErrorSeverity::Error
    } else {
      TpmErrorSeverity::Warning
    }
  }
  // Don't expose this wider, we only want to construct this from TpmError.
  pub(super) const fn new_unchecked(error: TpmError) -> Self {
    Self(error)
  }
  pub const fn new(value: u32) -> Option<Self> {
    // If it's format 1 then we should fail - it's not the right code.
    if value & RC_FMT1 != 0 {
      return None;
    }
    match TpmError::new(value) {
      Some(value) => Some(Self(value)),
      None => None,
    }
  }
  // Panics!
  pub const fn new_checked(value: u32) -> Self {
    match Self::new(value) {
      Some(value) => value,
      None => panic!("TpmFmt0Error cannot be 0"),
    }
  }
}

// =============================================================================
// TRAITS
// =============================================================================

// -----------------------------------------------------------------------------
impl Deref for TpmFmt0Error {
  type Target = TpmError;
  fn deref(&self) -> &Self::Target {
      &self.0
  }
}

// -----------------------------------------------------------------------------
impl DerefMut for TpmFmt0Error {
  fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.0
  }
}

// -----------------------------------------------------------------------------
impl From<TpmFmt0Error> for TpmError {
  fn from(value: TpmFmt0Error) -> Self {
    value.0
  }
}
