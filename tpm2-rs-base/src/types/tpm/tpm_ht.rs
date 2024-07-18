use super::*;

// TPM2HT represents a TPM_HT.
// See definition in Part 2: Structures, section 7.2.
#[open_enum]
#[repr(u8)]
pub enum TPM2HT {
    PCR = 0x00,
    NVIndex = 0x01,
    HMACSession = 0x02,
    PolicySession = 0x03,
    Permanent = 0x40,
    Transient = 0x80,
    Persistent = 0x81,
    AC = 0x90,
}
