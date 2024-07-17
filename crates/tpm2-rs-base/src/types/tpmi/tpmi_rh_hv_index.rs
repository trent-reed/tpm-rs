use super::*;

/// TpmiRhNvIndex represents an NV location (TPMI_RH_NV_INDEX).
/// See definition in Part 2: Structures, section 9.24.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug, Default, Marshalable)]
pub struct TpmiRhNvIndex(u32);
impl TryFrom<u32> for TpmiRhNvIndex {
    // TODO: This is probably wrong, because it's not an error from the TPM.
    type Error = TpmError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if TpmHc::is_nv_index(value) {
            Ok(TpmiRhNvIndex(value))
        } else {
            Err(TPM_RC_VALUE.into())
        }
    }
}
