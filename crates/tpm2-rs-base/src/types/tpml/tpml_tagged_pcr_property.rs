use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmlTaggedPcrProperty {
    count: u32,
    #[marshal(length=count)]
    pcr_property: [TpmsTaggedPcrSelect; TPM2_MAX_PCR_PROPERTIES],
}

impl_tpml! {TpmlTaggedPcrProperty, pcr_property, TpmsTaggedPcrSelect, TPM2_MAX_PCR_PROPERTIES}
