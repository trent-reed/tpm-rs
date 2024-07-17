use super::*;

fn expand_struct_unmarshal_assignments_length(ident: &Ident, field: &Field, length: &ExprPath) -> Result<TokenStream> {
  let Type::Array(array_type) = &field.ty else {
    return Err(Error::new(
      field.span(),
      "It's only valid to apply a #[marshal(length=...)] attribute to an array."
    ));
  };
  let array_element = &*array_type.elem;
  let array_length = &array_type.len;
  Ok(quote_spanned!(field.span()=>
    if #length as usize > #array_length {
      return Err(::tpm2_rs_marshal::__private::TPM_RC_SIZE.into());
    }
    let mut #ident = [#array_element::default(); #array_length];
    for entry in #ident.iter_mut().take(#length as usize) {
      *entry = <#array_element>::try_unmarshal(buffer)?;
    }
  ))
}

fn expand_struct_unmarshal_assignments_regular(ident: &Ident, field: &Field) -> TokenStream {
  let field_type = &field.ty;
  quote_spanned!(field.span()=>
    let #ident = <#field_type>::try_unmarshal(buffer)?;
  )
}

fn expand_struct_unmarshal_assignments(ident: &Ident, field: &Field) -> Result<TokenStream> {
  let attributes = FieldAttributes::new(&field.attrs)?;
  match &attributes.marshal.length {
    Some(length) => expand_struct_unmarshal_assignments_length(ident, field, length),
    None => Ok(expand_struct_unmarshal_assignments_regular(ident, field)),
  }
}

fn expand_struct_unmarshal_unnamed(fields: &FieldsUnnamed) -> Result<TokenStream> {
  let fields: Vec<_> = fields.unnamed.iter()
    .enumerate()
    .map(|(i,f)| (Ident::new(&format!("f{}", i), Span::call_site()), f))
    .collect();
  let field_names = fields.iter()
    .map(|(ident,_)| ident);
  let field_assignments: Vec<_> = fields.iter()
    .map(|(ident, field)| expand_struct_unmarshal_assignments(ident, field))
    .collect::<Result<_>>()?;
  Ok(quote!(
    #( #field_assignments; )*
    Ok(Self ( #( #field_names, )* ))
  ))
}

fn expand_struct_unmarshal_named(fields: &FieldsNamed) -> Result<TokenStream> {
  let fields: Vec<_> = fields.named.iter()
    .map(|f| f.ident.as_ref()
      .map(|ident| (ident, f))
      .ok_or_else(|| Error::new(f.span(), "No")))
    .collect::<Result<_>>()?;
  let field_names = fields.iter()
    .map(|(ident,_)| ident);
  let field_assignments: Vec<_> = fields.iter()
    .map(|(ident, field)| expand_struct_unmarshal_assignments(ident, field))
    .collect::<Result<_>>()?;
  Ok(quote!(
    #( #field_assignments; )*
    Ok(Self { #( #field_names, )* })
  ))
}

pub(super) fn expand_struct_unmarshal(data: &DataStruct) -> Result<TokenStream> {
  match &data.fields {
    Fields::Unnamed(fields) => expand_struct_unmarshal_unnamed(fields),
    Fields::Named(fields) => expand_struct_unmarshal_named(fields),
    Fields::Unit => Ok(quote!( Ok(Self) )),
  }
}
