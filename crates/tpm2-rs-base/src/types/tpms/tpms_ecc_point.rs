use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsEccPoint {
    pub x: Tpm2bEccParameter,
    pub y: Tpm2bEccParameter,
}
