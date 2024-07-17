use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsQuoteInfo {
    pub pcr_select: TpmlPcrSelection,
    pub pcr_digest: Tpm2bDigest,
}
