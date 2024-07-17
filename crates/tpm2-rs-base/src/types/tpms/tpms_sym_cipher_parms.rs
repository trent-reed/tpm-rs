use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsSymCipherParms {
    pub sym: TpmtSymDefObject,
}
