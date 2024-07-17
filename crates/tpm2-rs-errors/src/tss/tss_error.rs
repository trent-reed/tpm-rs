// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;

// =============================================================================
// Modules
// =============================================================================

// -----------------------------------------------------------------------------
mod layers;
pub use layers::*;

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
pub enum TssErrorCode {
  /// Non-specific failure.
  Fail = 0x002,
  /// One or more parameter is bad.
  BadParameter = 0x003,
  /// An internal SW error has been detected.
  InternalError = 0x004,
  /// Ran out of memory.
  OutOfMemory = 0x005,
  /// Not implemented.
  NotImplemented = 0x006,
  /// Key could not be registered because UUID has already registered.
  KeyAlreadyRegistered = 0x008,
  /// TPM returns with success but TSP/TCS notice that something is wrong.
  TpmUnexpected = 0x010,
  /// A communications error with the TPM has been detected.
  CommunicationFailure = 0x011,
  /// The operation has timed out.
  Timeout = 0x012,
  /// The action was canceled.
  Canceled = 0x016,
  /// The TPM does not support the requested feature.
  TpmUnsupportedFeature = 0x014,
}

// -----------------------------------------------------------------------------
#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
#[repr(transparent)]
pub struct TssError(NonZeroU32);

// =============================================================================
// IMPLEMENTATION
// =============================================================================

// -----------------------------------------------------------------------------
impl TssError {
  pub const fn as_u32(&self) -> u32 {
    self.0.get()
  }
  pub const fn layer(&self) -> u8 {
    ((self.as_u32() >> 12) & 0b1111) as u8
  }
  pub const fn os_code(&self) -> u16 {
    (self.as_u32() >> 16) as u16
  }
  pub const fn code(&self) -> u32 {
    self.as_u32() & 0b11111111111
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
  pub const fn new_layered(layer: TssLayer, error: TssErrorCode) -> Self {
    // Safe because the value will never be 0 since the layer is mixed in.
    Self::new_checked(((layer as u32) << 12) | (error as u32))
  }
}

// =============================================================================
// TRAITS
// =============================================================================

// -----------------------------------------------------------------------------
impl<T: Into<TpmError>> From<T> for TssError {
  fn from(orig: T) -> Self {
    // Into the TpmError -> Into the NonZeroU32.
    // TODO: See if we can just officially say a TpmError is a TssError.
    Self(orig.into().into())
  }
}
