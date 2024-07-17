use super::*;

/// The number of bits in an RSA key.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug, Default, Marshalable)]
pub struct TpmiRsaKeyBits(pub(crate) u16);
