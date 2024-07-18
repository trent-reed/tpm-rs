use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bName {
    size: u16,
    pub name: [u8; TpmuName::UNION_SIZE],
}

impl_try_marshalable_tpm2b_simple! {Tpm2bName, name}
