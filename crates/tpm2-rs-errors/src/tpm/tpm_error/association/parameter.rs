// =============================================================================
// TYPES
// =============================================================================

#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
#[repr(u32)]
pub enum TpmErrorParameter {
  X1 = 0x1,
  X2 = 0x2,
  X3 = 0x3,
  X4 = 0x4,
  X5 = 0x5,
  X6 = 0x6,
  X7 = 0x7,
  X8 = 0x8,
  X9 = 0x9,
  XA = 0xA,
  XB = 0xB,
  XC = 0xC,
  XD = 0xD,
  XE = 0xE,
  XF = 0xF,
}

// =============================================================================
// IMPLEMENTATION
// =============================================================================

impl TpmErrorParameter {
  pub(super) const fn from_format1_code(code: u32) -> Option<Self> {
    match (code>>8) & 0b1111 {
      0x1 => Some(Self::X1),
      0x2 => Some(Self::X2),
      0x3 => Some(Self::X3),
      0x4 => Some(Self::X4),
      0x5 => Some(Self::X5),
      0x6 => Some(Self::X6),
      0x7 => Some(Self::X7),
      0x8 => Some(Self::X8),
      0x9 => Some(Self::X9),
      0xA => Some(Self::XA),
      0xB => Some(Self::XB),
      0xC => Some(Self::XC),
      0xD => Some(Self::XD),
      0xE => Some(Self::XE),
      0xF => Some(Self::XF),
      _ => None,
    }
  }
}
