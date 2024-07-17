use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Default, Debug, Marshalable)]
pub struct TpmsTaggedPcrSelect {
    tag: TPM2PTPCR,
    size_of_select: u8,
    #[marshal(length=size_of_select)]
    pcr_select: [u8; TPM2_PCR_SELECT_MAX as usize],
}
