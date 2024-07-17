// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
#[repr(u8)]
pub enum TssLayer {
  /// An error returned from a TPM.
  Tpm = 0,
  /// An error returned from a TDDL (Trusted Device Driver Library).
  Tddl = 1,
  /// An error returned from a TCS (Trusted Core Service).
  Tcs = 2,
  /// An error returned from a TSP (Trusted Service Provider).
  Tsp = 3,
}

// -----------------------------------------------------------------------------
#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
#[repr(transparent)]
pub struct TssErrorLayer<const L: u8>(TssError);

// -----------------------------------------------------------------------------
// If const generics ever develop further, we can type this to be TssLayer.
pub type TssTpmError = TssErrorLayer<{ TssLayer::Tpm as u8 }>;
pub type TssTddlError = TssErrorLayer<{ TssLayer::Tddl as u8 }>;
pub type TssTcsError = TssErrorLayer<{ TssLayer::Tcs as u8 }>;
pub type TssTspError = TssErrorLayer<{ TssLayer::Tsp as u8 }>;

// =============================================================================
// IMPLEMENTATION
// =============================================================================

// -----------------------------------------------------------------------------
// Only allow access to functionality to build the approved error layers.
// -----------------------------------------------------------------------------
impl TssTpmError {
  pub const LAYER: TssLayer = TssLayer::Tpm;
  pub const fn new(error: TssErrorCode) -> Self {
    Self(TssError::new_layered(Self::LAYER, error))
  }
}
impl TssTddlError {
  pub const LAYER: TssLayer = TssLayer::Tddl;
  pub const fn new(error: TssErrorCode) -> Self {
    Self(TssError::new_layered(Self::LAYER, error))
  }
}
impl TssTcsError {
  pub const LAYER: TssLayer = TssLayer::Tcs;
  pub const fn new(error: TssErrorCode) -> Self {
    Self(TssError::new_layered(Self::LAYER, error))
  }
}
impl TssTspError {
  pub const LAYER: TssLayer = TssLayer::Tsp;
  pub const fn new(error: TssErrorCode) -> Self {
    Self(TssError::new_layered(Self::LAYER, error))
  }
}

// =============================================================================
// TRAITS
// =============================================================================

// -----------------------------------------------------------------------------
impl<const L: u8> Deref for TssErrorLayer<L> {
  type Target = TssError;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

// -----------------------------------------------------------------------------
impl<const L: u8> DerefMut for TssErrorLayer<L> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

// -----------------------------------------------------------------------------
impl<const L: u8> From<TssErrorLayer<L>> for TssError {
  fn from(orig: TssErrorLayer<L>) -> Self {
    orig.0
  }
}
