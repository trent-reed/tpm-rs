use super::*;

#[derive(Clone, Copy, PartialEq, Debug, Default, Marshalable)]
pub struct TpmsPcrSelection {
    pub hash: TpmiAlgHash,
    pub sizeof_select: u8,
    #[marshal(length=sizeof_select)]
    pub pcr_select: [u8; TPM2_PCR_SELECT_MAX as usize],
}
