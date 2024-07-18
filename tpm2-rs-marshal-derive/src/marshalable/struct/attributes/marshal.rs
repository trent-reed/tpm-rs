use super::*;

#[derive(Default)]
pub struct FieldMarshalAttributes {
  pub length: Option<ExprPath>,
}

impl FieldMarshalAttributes {
  fn merge_length(&mut self, attr: &Attribute, value: Expr) -> Result<()> {
    if self.length.is_some() {
      return Err(Error::new(attr.span(), "Duplicate length attribute for marshaled type."));
    }
    let Expr::Path(path) = &value else {
      return Err(Error::new(value.span(), "Expected `Path` expression."));
    };
    self.length = Some(path.clone());
    Ok(())
  }
  pub fn merge(&mut self, attr: &Attribute) -> Result<()> {
    let args: MetaNameValue = attr.parse_args()?;
    let Some(attr_key) = args.path.get_ident() else {
      return Err(Error::new(attr.span(), "Empty key for marshal attribute (expected format `#[marshal(key=value)]`)."));
    };

    if attr_key == "length" {
      self.merge_length(attr, args.value)
    } else {
      Err(Error::new(attr.span(), "Unsupported metavariable for marshalling."))
    }
  }
}
