use super::*;

mod repr;
use repr::*;

#[derive(Default)]
pub struct EnumAttributes {
  pub repr: EnumReprAttribute,
}

impl EnumAttributes {
  pub fn new(attrs: &[Attribute]) -> Result<Self> {
    let mut attributes = Self::default();
    for attr in attrs {
      let Some(ident) = attr.path().get_ident() else { continue };
      if ident == "repr" {
        attributes.repr.merge(attr)?;
      }
    }
    Ok(attributes)
  }
}
