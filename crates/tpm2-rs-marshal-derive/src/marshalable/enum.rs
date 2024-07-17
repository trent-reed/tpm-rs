use super::*;

mod attributes;
use attributes::*;
mod marshal;
use marshal::*;
mod unmarshal;
use unmarshal::*;

pub(super) fn expand_enum(input: &DeriveInput, data: &DataEnum) -> Result<TokenStream> {
  // Check the attributes of the enum (required for getting the selector type).
  let attributes = EnumAttributes::new(&input.attrs)?;

  // Ensure that there are at least some variants assigned to the enum.
  if data.variants.is_empty() {
    return Err(Error::new(input.span(), concat!(
      "Enums without any fields will not produce correct code for marshaling ",
      "(try adding some variants to the enum, or use an empty struct instead)."
    )));
  }

  // The enum must have one of the supported discriminant types.
  // The specification will define the specific bit depth of the discriminant.
  let Some(selector) = attributes.repr.layout else {
    return Err(Error::new(input.span(), concat!(
      "Enums need to have a defined representation for the discriminant ",
      "(try adding `#[repr(T))]` where T is one of u8, u16, u32, or u64)."
    )));
  };

  /*
  // The enum must be in repr(c) format so that the layout is precitable.
  // Note: Technically if it's `repr(prim)`, then it's also `repr(c)`;
  //       so this might be overkill to require it also to specify `repr(c)`.
  if attributes.repr.c == false {
    return Err(Error::new(input.span(), concat!(
      "Enums need use a C layout to ensure that the discriminant is always ",
      "the first few bytes (see: rust-lang/rfcs#2195)."
    )));
  }
  */

  // Create some representation of the match/if arms to repeat.
  let marshal_arms: Vec<_> = data.variants.iter()
    .map(|v| expand_enum_marshal(v))
    .collect::<Result<_>>()?;
  let unmarshal_if_blocks: Vec<_> = data.variants.iter()
    .map(|v| expand_enum_unmarshal(v))
    .collect::<Result<_>>()?;

  let ident = &input.ident;
  Ok(quote!(
    impl ::tpm2_rs_marshal::MarshalableEnum for #ident {
        type Selector = #selector;

        // This is explicitly allowed for enums with primitive representation.
        // https://doc.rust-lang.org/std/mem/fn.discriminant.html#accessing-the-numeric-value-of-the-discriminant.
        fn discriminant(&self) -> Self::Selector {
            unsafe { *<*const _>::from(self).cast::<Self::Selector>() }
        }

        fn try_marshal_variant(&self, buffer: &mut [u8]) -> ::tpm2_rs_marshal::__private::TpmResult<usize> {
            let mut written: usize = 0;
            match self {
              #( #marshal_arms, )*
            }
            Ok(written)
        }

        fn try_unmarshal_variant(selector: Self::Selector, buffer: &mut UnmarshalBuf) -> ::tpm2_rs_marshal::__private::TpmResult<Self> {
            // Because of open_enum, we can't use a match clause here.
            // We might be able to be smart about this and codegen differently.
            Ok(#( #unmarshal_if_blocks else )* {
              return Err(::tpm2_rs_marshal::__private::TPM_RC_SELECTOR.into());
            })
        }
    }

    impl ::tpm2_rs_marshal::Marshalable for #ident {
      fn try_marshal(&self, buffer: &mut [u8]) -> ::tpm2_rs_marshal::__private::TpmResult<usize> {
        let mut written: usize = 0;
        written += self.discriminant().try_marshal(&mut buffer[written..])?;
        written += self.try_marshal_variant(&mut buffer[written..])?;
        Ok(written)
      }

      fn try_unmarshal(buffer: &mut UnmarshalBuf) -> ::tpm2_rs_marshal::__private::TpmResult<Self> {
        let selector = <Self as ::tpm2_rs_marshal::MarshalableEnum>::Selector::try_unmarshal(buffer)?;
        Self::try_unmarshal_variant(selector, buffer)
      }
    }
  ))
}
