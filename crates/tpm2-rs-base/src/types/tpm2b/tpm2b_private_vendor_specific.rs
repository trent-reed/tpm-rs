use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bPrivateVendorSpecific {
    size: u16,
    pub buffer: [u8; TPM2_PRIVATE_VENDOR_SPECIFIC_BYTES as usize],
}

impl_try_marshalable_tpm2b_simple! {Tpm2bPrivateVendorSpecific, buffer}
