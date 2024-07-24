use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bDerive {
    size: u16,
    pub buffer: [u8; size_of::<TpmsDerive>()],
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsDerive {
    pub label: Tpm2bLabel,
    pub context: Tpm2bLabel,
}
