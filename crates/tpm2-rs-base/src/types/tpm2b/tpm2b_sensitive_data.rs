use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bSensitiveData {
    size: u16,
    pub buffer: [u8; TpmuSensitiveCreate::UNION_SIZE],
}

impl_try_marshalable_tpm2b_simple! {Tpm2bSensitiveData, buffer}
