use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tpm2bCreationData {
    size: u16,
    pub creation_data: [u8; size_of::<TpmsCreationData>()],
}

impl_try_marshalable_tpm2b_simple! {Tpm2bCreationData, creation_data}
impl_try_marshalable_tpm2b_struct! {Tpm2bCreationData, TpmsCreationData, creation_data}

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
