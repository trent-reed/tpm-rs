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
//   if selector == discriminant {
//     Self::Variant1 {
//       field1: Type1.try_unmarshal(buffer)?,
//       field2: Type2.try_unmarshal(buffer)?,
//       field3: Type3.try_unmarshal(buffer)?,
//     }
//   }
fn expand_enum_unmarshal_if_block_named_fields(
  variant: &Variant,
  discriminant: &Expr,
  fields: &FieldsNamed
) -> Result<TokenStream> {
  let ident = &variant.ident;
  let types = fields.named.iter().map(|f| &f.ty);
  let fields = fields.named.iter().map(|f| &f.ident);
  Ok(quote_spanned!(variant.span() =>
    if selector == #discriminant {
      Self::#ident { #( #fields: <#types>::try_unmarshal(buffer)?, )* }
    }
  ))
}

// Example Input:
//   enum Example {
//     Variant1(Type1, Type2, Type3),
//   }
//
// Should expand to:
//   if selector == discriminant {
//     Self::Variant1(
//       Type1.try_unmarshal(buffer)?,
//       Type2.try_unmarshal(buffer)?,
//       Type3.try_unmarshal(buffer)?
//     )
//   }
fn expand_enum_unmarshal_if_block_unnamed_fields(
  variant: &Variant,
  discriminant: &Expr,
  fields: &FieldsUnnamed
) -> Result<TokenStream> {
  let ident = &variant.ident;
  let types = fields.unnamed.iter().map(|f| &f.ty);
  Ok(quote_spanned!(variant.span() =>
    if selector == #discriminant {
      Self::#ident( #( <#types>::try_unmarshal(buffer)? ),* )
    }
  ))
}

// Example Input:
//   enum Example {
//     Variant1,
//   }
//
// Should expand to:
//   if selector == discriminant {
//     Self::Variant1
//   }
fn expand_enum_unmarshal_if_block_unit(
  variant: &Variant,
  discriminant: &Expr
) -> TokenStream {
  let ident = &variant.ident;
  quote_spanned!(variant.span() =>
    if selector == #discriminant {
      Self::#ident
    }
  )
}

// Returns a token stream of match arms for use in `try_unmarshal`.
// See the `expand_enum` function for how this is used.
pub(super) fn expand_enum_unmarshal(variant: &Variant) -> Result<TokenStream> {
  let Some((_, discriminant)) = &variant.discriminant else {
    return Err(Error::new(variant.span(), concat!(
      "Each element of an enum needs to have a well-defined discriminant. ",
      "(try adding `= N;` to the end of this enum, where N is a selector)."
    )));
  };

  match &variant.fields {
    Fields::Unnamed(fields) => expand_enum_unmarshal_if_block_unnamed_fields(variant, discriminant, fields),
    Fields::Named(fields) => expand_enum_unmarshal_if_block_named_fields(variant, discriminant, fields),
    Fields::Unit => Ok(expand_enum_unmarshal_if_block_unit(variant, discriminant)),
  }
}
