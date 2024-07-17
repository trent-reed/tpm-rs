// =============================================================================
// TYPES
// =============================================================================

#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
pub enum TpmErrorSession {
  X1 = 0x1,
  X2 = 0x2,
  X3 = 0x3,
  X4 = 0x4,
  X5 = 0x5,
  X6 = 0x6,
  X7 = 0x7,
}

// =============================================================================
// IMPLEMENTATION
// =============================================================================

impl TpmErrorSession {
  pub(super) const fn from_format1_code(code: u32) -> Option<Self> {
    match (code>>8) & 0b111 {
      0x1 => Some(Self::X1),
      0x2 => Some(Self::X2),
      0x3 => Some(Self::X3),
      0x4 => Some(Self::X4),
      0x5 => Some(Self::X5),
      0x6 => Some(Self::X6),
      0x7 => Some(Self::X7),
      _ => None,
    }
  }
}
