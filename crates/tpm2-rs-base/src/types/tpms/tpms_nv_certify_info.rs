use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsNvCertifyInfo {
    pub index_name: Tpm2bName,
    pub offset: u16,
    pub nv_contents: Tpm2bMaxNvBuffer,
}
