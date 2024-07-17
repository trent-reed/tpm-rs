// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;

// Some code will depend on acting on enums, as such we need a separate trait
// that will provide more functionality in the case of a marshalable enum.
pub trait MarshalableEnum: Marshalable {
  type Selector: Sized;
  fn discriminant(&self) -> Self::Selector;
  fn try_unmarshal_variant(selector: Self::Selector, buffer: &mut UnmarshalBuf) -> TpmResult<Self>;
  fn try_marshal_variant(&self, buffer: &mut [u8]) -> TpmResult<usize>;
}
