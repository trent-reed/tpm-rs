use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsCertifyInfo {
    pub name: Tpm2bName,
    pub qualified_name: Tpm2bName,
}
