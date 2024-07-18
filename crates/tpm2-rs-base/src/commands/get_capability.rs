use super::*;

#[derive(Clone, Copy, PartialEq, Marshalable)]
pub struct GetCapabilityCmd {
    pub capability: TPM2Cap,
    pub property: TPM2PT,
    pub property_count: u32,
}
impl TpmCommand for GetCapabilityCmd {
    const CMD_CODE: TPM2CC = TPM2CC::GetCapability;
    type Handles = ();
    type RespT = GetCapabilityResp;
    type RespHandles = ();
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct GetCapabilityResp {
    pub more_data: TpmiYesNo,
    pub capability_data: TpmsCapabilityData,
}
