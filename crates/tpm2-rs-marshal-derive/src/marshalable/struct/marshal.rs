use super::*;

fn expand_struct_marshal_expr_field_length(expr: &Expr, field: &Field, length: &ExprPath) -> Result<TokenStream> {
  let Type::Array(array_type) = &field.ty else {
    return Err(Error::new(
      field.span(),
      "It's only valid to apply a #[marshal(length=...)] attribute to an array."
    ));
  };
  let array_length = &array_type.len;
  Ok(quote_spanned!(field.span()=>
    if self.#length as usize > #array_length {
      return Err(::tpm2_rs_marshal::__private::TPM_RC_SIZE.into());
    }
    for entry in #expr.iter().take(self.#length as usize) {
      written += entry.try_marshal(&mut buffer[written..])?
    }
  ))
}

fn expand_struct_marshal_expr_field_regular(expr: &Expr, field: &Field) -> TokenStream {
  quote_spanned!(field.span()=>
    written += #expr.try_marshal(&mut buffer[written..])?;
  )
}

fn expand_struct_marshal_expr_field(expr: &Expr, field: &Field) -> Result<TokenStream> {
  let attributes = FieldAttributes::new(&field.attrs)?;
  match &attributes.marshal.length {
    Some(length) => expand_struct_marshal_expr_field_length(expr, field, length),
    None => Ok(expand_struct_marshal_expr_field_regular(expr, field)),
  }
}

fn expand_struct_marshal_expr(fields: &[(Expr, &Field)]) -> Result<TokenStream> {
  let field_assignments: Vec<_> = fields.iter()
    .map(|(ident, field)| expand_struct_marshal_expr_field(ident, field))
    .collect::<Result<_>>()?;
  Ok(quote!(
    let mut written: usize = 0;
    #( #field_assignments )*
    Ok(written)
  ))
}

fn expand_struct_marshal_unnamed(fields: &FieldsUnnamed) -> Result<TokenStream> {
  let fields: Vec<_> = fields.unnamed.iter()
    .enumerate()
    .map(|(i,f)| {
      let ident = LitInt::new(&i.to_string(), Span::call_site());
      let expr: Expr = parse_quote! { self.#ident };
      (expr, f)
    })
    .collect();
  expand_struct_marshal_expr(&fields)
}

fn expand_struct_marshal_named(fields: &FieldsNamed) -> Result<TokenStream> {
  let fields: Vec<_> = fields.named.iter()
    .map(|f| {
      let ident = &f.ident;
      let expr: Expr = parse_quote! { self.#ident };
      (expr, f)
    })
    .collect();
  expand_struct_marshal_expr(&fields)
}

pub(super) fn expand_struct_marshal(data: &DataStruct) -> Result<TokenStream> {
  match &data.fields {
    Fields::Unnamed(fields) => expand_struct_marshal_unnamed(fields),
    Fields::Named(fields) => expand_struct_marshal_named(fields),
    Fields::Unit => Ok(quote!( Ok(0) )),
  }
}
