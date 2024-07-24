use super::*;

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmlPcrSelection {
    count: u32,
    #[marshal(length=count)]
    pcr_selections: [TpmsPcrSelection; TPM2_NUM_PCR_BANKS as usize],
}

impl_tpml! {TpmlPcrSelection, pcr_selections, TpmsPcrSelection, TPM2_NUM_PCR_BANKS}


#[derive(Clone, Copy, PartialEq, Debug, Default, Marshalable)]
pub struct TpmsPcrSelection {
    pub hash: TpmiAlgHash,
    pub sizeof_select: u8,
    #[marshal(length=sizeof_select)]
    pub pcr_select: [u8; TPM2_PCR_SELECT_MAX as usize],
}
