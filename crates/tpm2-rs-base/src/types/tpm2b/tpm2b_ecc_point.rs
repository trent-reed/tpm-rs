use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bEccPoint {
    size: u16,
    pub point: [u8; size_of::<TpmsEccPoint>()],
}
