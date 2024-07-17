// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
mod tpma_algorithm;
pub use tpma_algorithm::*;
mod tpma_cc;
pub use tpma_cc::*;
mod tpma_locality;
pub use tpma_locality::*;
mod tpma_nv;
pub use tpma_nv::*;
mod tpma_object;
pub use tpma_object::*;
mod tpma_session;
pub use tpma_session::*;

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
