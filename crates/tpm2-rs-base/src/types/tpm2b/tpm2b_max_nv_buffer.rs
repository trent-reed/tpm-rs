use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bMaxNvBuffer {
    size: u16,
    pub buffer: [u8; TPM2_MAX_NV_BUFFER_SIZE as usize],
}

impl_try_marshalable_tpm2b_simple! {Tpm2bMaxNvBuffer, buffer}
