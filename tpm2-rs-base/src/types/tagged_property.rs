use super::*;

#[derive(Clone, Copy, PartialEq, Default, Debug, Marshalable)]
pub struct TpmsTaggedProperty {
    pub property: TPM2PT,
    pub value: u32,
}
