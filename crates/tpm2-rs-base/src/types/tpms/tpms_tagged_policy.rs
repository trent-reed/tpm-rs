use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable, Default)]
pub struct TpmsTaggedPolicy {
    handle: TPM2Handle,
    policy_hash: TpmtHa,
}