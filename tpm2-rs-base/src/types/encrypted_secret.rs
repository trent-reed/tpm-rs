use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bEncryptedSecret {
    size: u16,
    pub secret: [u8; TpmuEncryptedSecret::UNION_SIZE],
}

impl_try_marshalable_tpm2b_simple! {Tpm2bEncryptedSecret, secret}

#[derive(UnionSize)]
#[repr(C, u16)]
pub enum TpmuEncryptedSecret {
    Ecc([u8; size_of::<TpmsEccPoint>()]),
    Rsa([u8; TPM2_MAX_RSA_KEY_BYTES as usize]),
    Symmetric([u8; size_of::<Tpm2bDigest>()]),
    KeyedHash([u8; size_of::<Tpm2bDigest>()]),
}
