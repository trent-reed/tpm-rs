use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bIdObject {
    size: u16,
    pub credential: [u8; size_of::<TpmsIdObject>()],
}

impl_try_marshalable_tpm2b_simple! {Tpm2bIdObject, credential}
