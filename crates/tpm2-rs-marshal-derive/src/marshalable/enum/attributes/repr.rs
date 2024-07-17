use super::*;

#[derive(Default)]
pub struct EnumReprAttribute {
  // Whether or not there is a `#[repr(C)]`
  pub c: bool,

  // Whether or not there is a `#[repr(<u8, u16, u32, u64>)]`
  // This doesn't make much sense outside of enums in our case.
  pub layout: Option<LayoutRepr>,
}

impl EnumReprAttribute {
  fn set_layout(&mut self, span: Span, repr: LayoutRepr) -> Result<()> {
    match self.layout {
      Some(other) => if other != repr {
        return Err(Error::new(span, "Duplicate incompatible repr provided."))
      },
      None => self.layout = Some(repr),
    }
    Ok(())
  }
  pub fn merge(&mut self, attr: &Attribute) -> Result<()> {
    attr.parse_nested_meta(|meta| {
      let Some(ident) = meta.path.get_ident() else {
        return Err(Error::new(attr.span(), "Empty path for meta attribute."));
      };

      if ident == "C" {
        self.c = true;
        Ok(())
      } else if ident == "u8" {
        self.set_layout(attr.span(), LayoutRepr::U8)
      } else if ident == "u16" {
        self.set_layout(attr.span(), LayoutRepr::U16)
      } else if ident == "u32" {
        self.set_layout(attr.span(), LayoutRepr::U32)
      } else if ident == "u64" {
        self.set_layout(attr.span(), LayoutRepr::U64)
      } else {
        Err(Error::new(attr.span(), "Unsupported representation for marshalling."))
      }
    })
  }
}
