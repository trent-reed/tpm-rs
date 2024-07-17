use super::*;

/// TpmiEccCurve represents an implemented ECC curve (TPMI_ECC_SCHEME).
/// See definition in Part 2: Structures, section 11.2.5.5.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug, Default, Marshalable)]
pub struct TpmiEccCurve(TPM2ECCCurve);
