use super::*;

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmlCc {
    count: u32,
    #[marshal(length=count)]
    command_codes: [TPM2CC; TPM2_MAX_CAP_CC],
}

impl_tpml! {TpmlCc, command_codes, TPM2CC, TPM2_MAX_CAP_CC}
