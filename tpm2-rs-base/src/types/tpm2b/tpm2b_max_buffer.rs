use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bMaxBuffer {
    size: u16,
    pub buffer: [u8; TPM2_MAX_DIGEST_BUFFER as usize],
}

impl_try_marshalable_tpm2b_simple! {Tpm2bMaxBuffer, buffer}
