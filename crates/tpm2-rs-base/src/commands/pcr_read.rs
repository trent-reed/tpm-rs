use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct PcrReadCmd {
    pcr_selection_in: TpmlPcrSelection,
}
impl TpmCommand for PcrReadCmd {
    const CMD_CODE: TPM2CC = TPM2CC::PCRRead;
    type Handles = ();
    type RespT = PcrReadResp;
    type RespHandles = ();
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct PcrReadResp {
    pcr_update_counter: u32,
    pcr_selection_out: TpmlPcrSelection,
    pcr_values: TpmlDigest,
}
