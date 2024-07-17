use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Marshalable)]
pub struct StartupCmd {
    pub startup_type: TPM2SU,
}
impl TpmCommand for StartupCmd {
    const CMD_CODE: TPM2CC = TPM2CC::Startup;
    type Handles = ();
    type RespT = ();
    type RespHandles = ();
}
