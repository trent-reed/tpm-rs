use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bContextData {
    size: u16,
    pub buffer: [u8; size_of::<TpmsContextData>()],
}

impl_try_marshalable_tpm2b_simple! {Tpm2bContextData, buffer}
