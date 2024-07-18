use super::*;

#[derive(UnionSize)]
#[repr(C, u16)]
pub enum TpmuName {
    Digest(TpmtHa),
    Handle(TPM2Handle),
}
