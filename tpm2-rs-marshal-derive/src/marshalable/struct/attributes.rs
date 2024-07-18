use super::*;

mod marshal;
pub use marshal::*;

#[derive(Default)]
pub struct FieldAttributes {
  pub marshal: FieldMarshalAttributes,
}

impl FieldAttributes {
  pub fn new(attrs: &[Attribute]) -> Result<Self> {
    let mut attributes = Self::default();
    for attr in attrs {
      let Some(ident) = attr.path().get_ident() else { continue };
      if ident == "marshal" {
        attributes.marshal.merge(attr)?;
      }
    }
    Ok(attributes)
  }
}
