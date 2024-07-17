// =============================================================================
// USES
// =============================================================================

use super::*;

// =============================================================================
// MODULES
// =============================================================================
mod r#enum;
use r#enum::*;
mod layout_repr;
use layout_repr::*;
mod r#struct;
use r#struct::*;

pub(super) fn expand_derive_macro(input: &DeriveInput) -> Result<TokenStream> {
  match &input.data {
      Data::Enum(data) => expand_enum(input, data),
      Data::Struct(data) => expand_struct(input, data),
      Data::Union(_) => Err(Error::new(
          input.span(),
          "Marshalable cannot be derived for union type",
      )),
  }
}
