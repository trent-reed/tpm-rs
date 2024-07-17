use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsTimeAttestInfo {
    pub time: TpmsTimeInfo,
    pub firmware_version: u64,
}
