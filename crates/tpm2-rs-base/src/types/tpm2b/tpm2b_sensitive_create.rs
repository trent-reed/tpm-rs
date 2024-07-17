use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bSensitiveCreate {
    size: u16,
    pub sensitive: [u8; size_of::<TpmsSensitiveCreate>()],
}

impl_try_marshalable_tpm2b_simple! {Tpm2bSensitiveCreate, sensitive}
impl_try_marshalable_tpm2b_struct! {Tpm2bSensitiveCreate, TpmsSensitiveCreate, sensitive}
