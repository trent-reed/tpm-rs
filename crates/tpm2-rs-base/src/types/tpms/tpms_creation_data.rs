use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsCreationData {
    pub pcr_select: TpmlPcrSelection,
    pub pcr_digest: Tpm2bDigest,
    pub locality: TpmaLocality,
    pub parent_name_alg: TPM2AlgID,
    pub parent_name: Tpm2bName,
    pub parent_qualified_name: Tpm2bName,
    pub outside_info: Tpm2bData,
}
