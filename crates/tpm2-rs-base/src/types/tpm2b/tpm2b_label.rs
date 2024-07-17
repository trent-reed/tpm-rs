use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bLabel {
    size: u16,
    pub buffer: [u8; TPM2_LABEL_MAX_BUFFER as usize],
}

impl_try_marshalable_tpm2b_simple! {Tpm2bLabel, buffer}
