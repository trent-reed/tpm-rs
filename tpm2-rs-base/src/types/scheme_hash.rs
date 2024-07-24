use super::*;

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsSchemeHash {
    pub hash_alg: TpmiAlgHash,
}
