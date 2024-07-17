use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsTimeInfo {
    pub time: u64,
    pub clock_info: TpmsClockInfo,
}
