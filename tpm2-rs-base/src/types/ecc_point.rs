use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bEccPoint {
    size: u16,
    pub point: [u8; size_of::<TpmsEccPoint>()],
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsEccPoint {
    pub x: Tpm2bEccParameter,
    pub y: Tpm2bEccParameter,
}
