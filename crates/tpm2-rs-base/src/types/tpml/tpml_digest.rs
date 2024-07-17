use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmlDigest {
    count: u32,
    #[marshal(length=count)]
    digests: [Tpm2bDigest; TPML_DIGEST_MAX_DIGESTS],
}

impl_tpml! {TpmlDigest, digests, Tpm2bDigest, TPML_DIGEST_MAX_DIGESTS}
