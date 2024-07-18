use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bAttest {
    size: u16,
    pub attestation_data: [u8; size_of::<TpmsAttest>()],
}

impl_try_marshalable_tpm2b_simple! {Tpm2bAttest, attestation_data}
