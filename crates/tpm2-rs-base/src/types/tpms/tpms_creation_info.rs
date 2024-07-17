use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsCreationInfo {
    pub object_name: Tpm2bName,
    pub creation_hash: Tpm2bDigest,
}
