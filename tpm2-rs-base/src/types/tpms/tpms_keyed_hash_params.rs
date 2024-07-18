use super::*;

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsKeyedHashParms {
    pub scheme: TpmtKeyedHashScheme,
}
