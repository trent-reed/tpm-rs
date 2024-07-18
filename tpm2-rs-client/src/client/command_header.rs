use super::*;
use tpm2_rs_base::TpmiStCommandTag;
use tpm2_rs_base::TPM2CC;

#[derive(Clone, Copy, PartialEq, Marshalable)]
pub struct CmdHeader {
    pub tag: TpmiStCommandTag,
    pub size: u32,
    pub code: TPM2CC,
}
impl CmdHeader {
    pub fn new(has_sessions: bool, code: TPM2CC) -> CmdHeader {
        let tag = if has_sessions {
            TpmiStCommandTag::NoSessions
        } else {
            TpmiStCommandTag::Sessions
        };
        CmdHeader { tag, size: 0, code }
    }
}
