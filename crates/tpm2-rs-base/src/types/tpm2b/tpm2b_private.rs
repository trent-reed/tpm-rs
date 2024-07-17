use super::*;

// TODO: Unsure how useful this is if it's a private struct.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
struct _PRIVATE {
    integrity_outer: Tpm2bDigest,
    integrity_inner: Tpm2bDigest,
    sensitive: Tpm2bSensitive,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bPrivate {
    size: u16,
    pub buffer: [u8; size_of::<_PRIVATE>()],
}

impl_try_marshalable_tpm2b_simple! {Tpm2bPrivate, buffer}
