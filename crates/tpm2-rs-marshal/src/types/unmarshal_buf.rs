pub struct UnmarshalBuf<'a> {
  buffer: &'a [u8],
}

impl<'a> UnmarshalBuf<'a> {
  pub fn new(buffer: &'a [u8]) -> UnmarshalBuf<'a> {
      UnmarshalBuf { buffer }
  }

  pub fn get(&mut self, len: usize) -> Option<&'a [u8]> {
      if len > self.buffer.len() {
          None
      } else {
          let (yours, mine) = self.buffer.split_at(len);
          self.buffer = mine;
          Some(yours)
      }
  }

  pub fn len(&self) -> usize {
      self.buffer.len()
  }

  pub fn is_empty(&self) -> bool {
      self.buffer.is_empty()
  }
}
