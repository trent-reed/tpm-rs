use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bPrivateKeyRsa {
    size: u16,
    // TODO: Why /2 ??? Should be documented or a different constant.
    pub buffer: [u8; (TPM2_MAX_RSA_KEY_BYTES / 2) as usize],
}

impl_try_marshalable_tpm2b_simple! {Tpm2bPrivateKeyRsa, buffer}
