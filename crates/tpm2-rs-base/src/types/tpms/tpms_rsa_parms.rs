use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsRsaParms {
    pub symmetric: TpmtSymDefObject,
    pub scheme: TpmtRsaScheme,
    pub key_bits: TpmiRsaKeyBits,
    pub exponent: u32,
}
