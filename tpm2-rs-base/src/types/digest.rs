use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bDigest {
    size: u16,
    pub buffer: [u8; TpmtHa::UNION_SIZE],
}

impl_try_marshalable_tpm2b_simple! {Tpm2bDigest, buffer}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmlDigest {
    count: u32,
    #[marshal(length=count)]
    digests: [Tpm2bDigest; TPML_DIGEST_MAX_DIGESTS],
}

impl_tpml! {TpmlDigest, digests, Tpm2bDigest, TPML_DIGEST_MAX_DIGESTS}
