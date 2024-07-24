use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bEvent {
    size: u16,
    pub buffer: [u8; 1024],
}

impl_try_marshalable_tpm2b_simple! {Tpm2bEvent, buffer}
