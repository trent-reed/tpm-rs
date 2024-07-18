use super::*;

// Example Input:
//   enum Example {
//     Variant1 {
//       field1: Type1,
//       field2: Type2,
//       field3: Type3,
//     },
//   }
//
// Should expand to:
//   Self::Variant1 { field1, field2, field3 } => {
//     written += field1.marshal(...)?;
//     written += field2.marshal(...)?;
//     written += field3.marshal(...)?;
//   }
fn expand_enum_marshal_match_arm_named_fields(
  variant: &Variant,
  fields: &FieldsNamed
) -> Result<TokenStream> {
  let ident = &variant.ident;
  let fields: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
  Ok(quote_spanned!(variant.span() =>
    Self::#ident { #(#fields),* } => {
      #( written += #fields.try_marshal(&mut buffer[written..])?; )*
    }
  ))
}

// Example Input:
//   enum Example {
//     Variant1(Type1, Type2, Type3),
//   }
//
// Should expand to:
//   Self::Variant1(field1, field2, field3) => {
//     written += field1.marshal(...)?;
//     written += field2.marshal(...)?;
//     written += field3.marshal(...)?;
//   }
fn expand_enum_marshal_match_arm_unnamed_fields(
  variant: &Variant,
  fields: &FieldsUnnamed
) -> Result<TokenStream> {
  let ident = &variant.ident;
  let fields: Vec<_> = fields.unnamed.iter()
    .enumerate()
    .map(|(i,_)| Ident::new(&format!("f{}", i), Span::call_site()))
    .collect();
  Ok(quote_spanned!(variant.span() =>
    Self::#ident( #(#fields),* ) => {
      #( written += #fields.try_marshal(&mut buffer[written..])?; )*
    }
  ))
}

// Example Input:
//   enum Example {
//     Variant1,
//   }
//
// Should expand to:
//   Self::Variant1 => (),
fn expand_enum_marshal_match_arm_unit(variant: &Variant) -> TokenStream {
  let ident = &variant.ident;
  quote_spanned!(variant.span() =>
    Self::#ident => ()
  )
}

// Returns a token stream of match arms for use in `try_marshal`.
// See the `expand_enum` function for how this is used.
pub(super) fn expand_enum_marshal(variant: &Variant) -> Result<TokenStream> {
  match &variant.fields {
    Fields::Unnamed(fields) => expand_enum_marshal_match_arm_unnamed_fields(variant, fields),
    Fields::Named(fields) => expand_enum_marshal_match_arm_named_fields(variant, fields),
    Fields::Unit => Ok(expand_enum_marshal_match_arm_unit(variant)),
  }
}
