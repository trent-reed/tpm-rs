use super::*;

/// The number of bits in an SM4 key.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug, Default, Marshalable)]
pub struct TpmiSm4KeyBits(u16);
