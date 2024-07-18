use super::*;

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmlTaggedTpmProperty {
    count: u32,
    #[marshal(length=count)]
    tpm_property: [TpmsTaggedProperty; TPM2_MAX_TPM_PROPERTIES],
}

impl_tpml! {TpmlTaggedTpmProperty, tpm_property, TpmsTaggedProperty, TPM2_MAX_TPM_PROPERTIES}
