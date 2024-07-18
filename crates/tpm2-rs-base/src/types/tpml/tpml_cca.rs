use super::*;

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmlCca {
    count: u32,
    #[marshal(length=count)]
    command_attributes: [TpmaCc; TPM2_MAX_CAP_CC],
}
