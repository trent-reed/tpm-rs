use super::*;

mod attributes;
use attributes::*;
mod marshal;
use marshal::*;
mod unmarshal;
use unmarshal::*;

pub(super) fn expand_struct(input: &DeriveInput, data: &DataStruct) -> Result<TokenStream> {
  let ident = &input.ident;
  let marshal_impl = expand_struct_marshal(data)?;
  let unmarshal_impl = expand_struct_unmarshal(data)?;
  Ok(quote!(
    impl ::tpm2_rs_marshal::Marshalable for #ident {
      fn try_marshal(&self, buffer: &mut [u8]) -> ::tpm2_rs_marshal::__private::TssTspResult<usize> {
        #marshal_impl
      }

      fn try_unmarshal(buffer: &mut UnmarshalBuf) -> ::tpm2_rs_marshal::__private::TssTspResult<Self> {
        #unmarshal_impl
      }
    }
  ))
}
