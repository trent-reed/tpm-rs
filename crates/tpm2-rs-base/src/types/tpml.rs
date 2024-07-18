// =============================================================================
// MACROS
// =============================================================================

// -----------------------------------------------------------------------------
// TODO: See if we really need this macro.
macro_rules! impl_tpml {
  ($T:ty,  $ListField:ident, $ListType:ty, $ListCapacity:ident) => {
      // Implement Default for the type. This cannot usually be derived, because $ListCapacity is too large.
      impl Default for $T {
          fn default() -> Self {
              Self {
                  count: 0,
                  $ListField: [<$ListType>::default(); $ListCapacity as usize],
              }
          }
      }

      impl $T {
          pub fn new(elements: &[$ListType]) -> TssTspResult<$T> {
              if elements.len() > $ListCapacity as usize {
                  return Err(TssTspError::new(TssErrorCode::InternalError));
              }
              let mut x = Self::default();
              x.count = elements.len() as u32;
              x.$ListField[..elements.len()].copy_from_slice(elements);
              Ok(x)
          }

          pub fn add(&mut self, element: &$ListType) -> TssTspResult<()> {
              if self.count() >= self.$ListField.len() {
                return Err(TssTspError::new(TssErrorCode::InternalError));
              }
              self.$ListField[self.count()] = *element;
              self.count += 1;
              Ok(())
          }

          pub fn count(&self) -> usize {
              self.count as usize
          }

          pub fn $ListField(&self) -> &[$ListType] {
              &self.$ListField[..min(self.count(), $ListCapacity as usize)]
          }
      }
  };
}

// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
mod tpml_alg_property;
pub use tpml_alg_property::*;
mod tpml_cc;
pub use tpml_cc::*;
mod tpml_cca;
pub use tpml_cca::*;
mod tpml_digest;
pub use tpml_digest::*;
mod tpml_ecc_curve;
pub use tpml_ecc_curve::*;
mod tpml_handle;
pub use tpml_handle::*;
mod tpml_pcr_selection;
pub use tpml_pcr_selection::*;
mod tpml_tagged_pcr_property;
pub use tpml_tagged_pcr_property::*;
mod tpml_tagged_policy;
pub use tpml_tagged_policy::*;
mod tpml_tagged_tpm_property;
pub use tpml_tagged_tpm_property::*;

// =============================================================================
// COMMON USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;
