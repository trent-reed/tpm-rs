use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bPublic {
    size: u16,
    pub public_area: [u8; size_of::<TpmtPublic>()],
}

impl_try_marshalable_tpm2b_simple! {Tpm2bPublic, public_area}
impl_try_marshalable_tpm2b_struct! {Tpm2bPublic, TpmtPublic, public_area}
