use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bSensitiveCreate {
    size: u16,
    pub sensitive: [u8; size_of::<TpmsSensitiveCreate>()],
}

impl_try_marshalable_tpm2b_simple! {Tpm2bSensitiveCreate, sensitive}
impl_try_marshalable_tpm2b_struct! {Tpm2bSensitiveCreate, TpmsSensitiveCreate, sensitive}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsSensitiveCreate {
    pub user_auth: Tpm2bAuth,
    pub data: Tpm2bSensitiveData,
}

#[derive(UnionSize)]
#[repr(C, u16)]
pub enum TpmuSensitiveCreate {
    Create([u8; TPM2_MAX_SYM_DATA as usize]),
    Derive(TpmsDerive),
}
