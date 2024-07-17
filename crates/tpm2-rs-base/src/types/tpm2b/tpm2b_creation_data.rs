use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bCreationData {
    size: u16,
    pub creation_data: [u8; size_of::<TpmsCreationData>()],
}

impl_try_marshalable_tpm2b_simple! {Tpm2bCreationData, creation_data}
impl_try_marshalable_tpm2b_struct! {Tpm2bCreationData, TpmsCreationData, creation_data}
