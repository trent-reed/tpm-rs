use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsSchemeXor {
    pub hash_alg: TpmiAlgHash,
    pub kdf: TpmiAlgKdf,
}
