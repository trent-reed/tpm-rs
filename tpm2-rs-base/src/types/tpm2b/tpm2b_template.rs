use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bTemplate {
    size: u16,
    pub buffer: [u8; size_of::<TpmtPublic>()],
}

impl_try_marshalable_tpm2b_simple! {Tpm2bTemplate, buffer}
