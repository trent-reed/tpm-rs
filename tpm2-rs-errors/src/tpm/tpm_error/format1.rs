// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;

// =============================================================================
// TYPES
// =============================================================================

// Format1 always has an association.
#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
#[repr(transparent)]
pub struct TpmFmt1Error(TpmError);

// =============================================================================
// IMPLEMENTATION
// =============================================================================

impl TpmFmt1Error {
  pub const fn data(&self) -> u32 {
    self.0.data()
  }
  pub const fn as_u32(&self) -> u32 {
    self.0.as_u32()
  }
  pub const fn code(&self) -> u8 {
    (self.as_u32() & 0b111111) as u8
  }
  pub const fn version(&self) -> TpmErrorVersion {
    // This format is always specific to TPM2.
    // The 8th bit is used for some other purposes here.
    TpmErrorVersion::Tpm2
  }
  pub const fn format(&self) -> TpmErrorFormat {
    // We can only construct this type if it's for sure the right format.
    // As such, no need to check the actual data, just return the format.
    TpmErrorFormat::One
  }
  pub fn association(&self) -> Option<TpmErrorAssociation> {
    TpmErrorAssociation::from_format1_code(self.as_u32())
  }
  // Don't expose this wider, we only want to construct this from TpmError.
  pub(super) const fn new_unchecked(error: TpmError) -> Self {
    Self(error)
  }
  pub const fn new(value: u32) -> Option<Self> {
    // If it's not format 1 then we should fail - it's not the right code.
    if value & RC_FMT1 == 0 {
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
      None => panic!("TpmRcFmt0Error cannot be 0"),
    }
  }
}

// =============================================================================
// TRAITS
// =============================================================================

// -----------------------------------------------------------------------------
impl Deref for TpmFmt1Error {
  type Target = TpmError;
  fn deref(&self) -> &Self::Target {
      &self.0
  }
}

// -----------------------------------------------------------------------------
impl DerefMut for TpmFmt1Error {
  fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.0
  }
}

// -----------------------------------------------------------------------------
impl From<TpmFmt1Error> for TpmError {
  fn from(value: TpmFmt1Error) -> Self {
    value.0
  }
}
