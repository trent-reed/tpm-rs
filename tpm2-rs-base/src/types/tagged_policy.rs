use super::*;

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmlTaggedPolicy {
    count: u32,
    #[marshal(length=count)]
    policies: [TpmsTaggedPolicy; TPM2_MAX_TAGGED_POLICIES],
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable, Default)]
pub struct TpmsTaggedPolicy {
    handle: TPM2Handle,
    policy_hash: TpmtHa,
}
