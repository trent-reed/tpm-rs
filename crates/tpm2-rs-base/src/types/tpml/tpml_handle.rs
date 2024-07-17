use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmlHandle {
    count: u32,
    #[marshal(length=count)]
    handle: [TPM2Handle; TPM2_MAX_CAP_HANDLES],
}

impl_tpml! {TpmlHandle, handle, TPM2Handle, TPM2_MAX_CAP_HANDLES}
