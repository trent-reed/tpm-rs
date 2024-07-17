use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bSymKey {
    size: u16,
    pub buffer: [u8; TPM2_MAX_SYM_KEY_BYTES as usize],
}

impl_try_marshalable_tpm2b_simple! {Tpm2bSymKey, buffer}
