use super::*;

/// The number of bits in a Camellia key.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug, Default, Marshalable)]
pub struct TpmiCamelliaKeyBits(u16);
