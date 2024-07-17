use super::*;

#[derive(UnionSize)]
#[repr(C, u16)]
pub enum TpmuSensitiveCreate {
    Create([u8; TPM2_MAX_SYM_DATA as usize]),
    Derive(TpmsDerive),
}
