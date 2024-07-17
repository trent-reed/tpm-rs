use super::*;

// TPM2SE represents a TPM_SE.
// See definition in Part 2: Structures, section 6.11.
#[open_enum]
#[repr(u8)]
pub enum TPM2SE {
    HMAC = 0x00,
    Policy = 0x01,
    Trial = 0x03,
}
