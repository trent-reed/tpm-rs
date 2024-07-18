use super::*;

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsIdObject {
    pub integrity_hmac: Tpm2bDigest,
    pub enc_identity: Tpm2bDigest,
}
