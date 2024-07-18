use super::*;

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsContextData {
    pub integrity: Tpm2bDigest,
    pub encrypted: Tpm2bContextSensitive,
}
