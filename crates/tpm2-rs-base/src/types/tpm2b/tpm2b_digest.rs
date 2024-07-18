use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bDigest {
    size: u16,
    pub buffer: [u8; TpmtHa::UNION_SIZE],
}

impl_try_marshalable_tpm2b_simple! {Tpm2bDigest, buffer}
