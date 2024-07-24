use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bNvPublic {
    size: u16,
    pub nv_public: [u8; size_of::<TpmsNvPublic>()],
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsNvPublic {
    pub nv_index: TpmiRhNvIndex,
    pub name_alg: TpmiAlgHash,
    pub attributes: TpmaNv,
    pub auth_policy: Tpm2bDigest,
    pub data_size: u16,
}
