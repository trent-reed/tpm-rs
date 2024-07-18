use super::*;
use tpm2_rs_base::TPM2ST;

#[derive(Clone, Copy, PartialEq, Marshalable, Debug)]
pub struct RespHeader {
    pub tag: TPM2ST,
    pub size: u32,
    pub rc: u32,
}
