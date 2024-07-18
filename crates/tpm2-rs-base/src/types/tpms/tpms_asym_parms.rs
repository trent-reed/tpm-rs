use super::*;

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsAsymParms {
    pub symmetric: TpmtSymDefObject,
    pub scheme: TpmtAsymScheme,
}
