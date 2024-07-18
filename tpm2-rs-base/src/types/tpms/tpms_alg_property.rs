use super::*;

#[derive(Clone, Copy, PartialEq, Default, Debug, Marshalable)]
pub struct TpmsAlgProperty {
    pub alg: TPM2AlgID,
    pub alg_properties: TpmaAlgorithm,
}
