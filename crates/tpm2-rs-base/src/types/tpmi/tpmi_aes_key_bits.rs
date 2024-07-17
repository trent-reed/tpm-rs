use super::*;

/// The number of bits in an AES key.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug, Default, Marshalable)]
pub struct TpmiAesKeyBits(u16);
