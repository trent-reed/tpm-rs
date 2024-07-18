use super::*;

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmlEccCurve {
    count: u32,
    #[marshal(length=count)]
    ecc_curves: [TPM2ECCCurve; TPM2_MAX_ECC_CURVES],
}

impl_tpml! {TpmlEccCurve, ecc_curves, TPM2ECCCurve, TPM2_MAX_ECC_CURVES}
