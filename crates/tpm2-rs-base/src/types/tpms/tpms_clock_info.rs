use super::*;

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsClockInfo {
    pub clock: u64,
    pub reset_count: u32,
    pub restart_count: u32,
    pub safe: TpmiYesNo,
}
