use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsKeyedHashParms {
    pub scheme: TpmtKeyedHashScheme,
}