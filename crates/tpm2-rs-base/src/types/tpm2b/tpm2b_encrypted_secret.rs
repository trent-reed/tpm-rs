use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bEncryptedSecret {
    size: u16,
    pub secret: [u8; TpmuEncryptedSecret::UNION_SIZE],
}

impl_try_marshalable_tpm2b_simple! {Tpm2bEncryptedSecret, secret}
