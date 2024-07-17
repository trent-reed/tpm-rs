use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmlPcrSelection {
    count: u32,
    #[marshal(length=count)]
    pcr_selections: [TpmsPcrSelection; TPM2_NUM_PCR_BANKS as usize],
}

impl_tpml! {TpmlPcrSelection, pcr_selections, TpmsPcrSelection, TPM2_NUM_PCR_BANKS}
