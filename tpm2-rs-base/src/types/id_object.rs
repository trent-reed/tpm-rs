use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bIdObject {
    size: u16,
    pub credential: [u8; size_of::<TpmsIdObject>()],
}

impl_try_marshalable_tpm2b_simple! {Tpm2bIdObject, credential}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsIdObject {
    pub integrity_hmac: Tpm2bDigest,
    pub enc_identity: Tpm2bDigest,
}
