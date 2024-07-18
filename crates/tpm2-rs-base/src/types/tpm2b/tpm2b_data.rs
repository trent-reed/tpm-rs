use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bData {
    size: u16,
    pub buffer: [u8; TpmtHa::UNION_SIZE],
}

impl_try_marshalable_tpm2b_simple! {Tpm2bData, buffer}
