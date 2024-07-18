// =============================================================================
// ATTRIBUTES
// =============================================================================

#![forbid(unsafe_code)]

// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
// PRIVATE MODULES
//   Never mark any of these modules as `pub`, we don't want to expose them.
// -----------------------------------------------------------------------------
mod prelude;
pub(crate) use prelude::*;  // Include the prelude for the crate.

// -----------------------------------------------------------------------------
// PRIVATE MODULES
//   Never mark any of these modules as `pub`, we don't want to expose them.
// -----------------------------------------------------------------------------
mod marshalable;

// -----------------------------------------------------------------------------
// PROCEDURAL MACROS
//   These are the procedural macros defined by this proc macro crate.
// -----------------------------------------------------------------------------

/// The Marshal derive macro generates an implementation of the Marshalable trait
/// for a struct by calling try_{un}marshal on each field in the struct. This
/// requires that the type of each field in the struct meets one of the
/// following conditions:
///  - The type is an array, the array entry type also meets these Marshal
///    conditions, and the array field is tagged with the #[marshal(length = $length_field)]
///    attribute, where $length_field is a field in the struct appearing before
///    the array field that can be converted to usize. In this case, the
///    generated code will {un}marshal first N entries in the array, where N is
///    the value of $length_field.
///  - The type is an enum type with #[repr(C, $primitive)] representation. The
///    generated code will include a discriminant() implementation that returns
///    $primitive, try_{un}marshal routines that accept an external selector, and will
///    {un}marshal the discriminant in BE format prior to the variant.
#[proc_macro_derive(Marshalable, attributes(marshal))]
pub fn derive_tpm_marshal(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    marshalable::expand_derive_macro(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
