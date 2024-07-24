use super::*;

#[derive(Clone, Copy, PartialEq, Default, Debug, Marshalable)]
pub struct TpmsAlgProperty {
    pub alg: TPM2AlgID,
    pub alg_properties: TpmaAlgorithm,
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmlAlgProperty {
    count: u32,
    #[marshal(length=count)]
    alg_properties: [TpmsAlgProperty; TPM2_MAX_CAP_ALGS],
}

impl_tpml! {TpmlAlgProperty, alg_properties, TpmsAlgProperty, TPM2_MAX_CAP_ALGS}
