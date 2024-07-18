use super::*;

#[derive(Copy,Clone,Eq,PartialEq)]
pub enum LayoutRepr {
  U8,
  U16,
  U32,
  U64,
}

impl LayoutRepr {
  pub fn as_str(&self) -> &str {
    match self {
      Self::U8 => "u8",
      Self::U16 => "u16",
      Self::U32 => "u32",
      Self::U64 => "u64",
    }
  }
}

impl ToTokens for LayoutRepr {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    Ident::new(self.as_str(), Span::call_site()).to_tokens(tokens)
  }
}
