use super::*;

// TPM2PTPCR represents a TPM_PT_PCR.
// See definition in Part 2: Structures, section 6.14.
#[open_enum]
#[repr(u32)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, Default, Marshalable)]
pub enum TPM2PTPCR {
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR is saved and
    // restored by TPM_SU_STATE
    Save = 0x00000000,
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be
    // extended from locality 0
    ExtendL0 = 0x00000001,
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be reset
    // by TPM2_PCR_Reset() from locality 0
    ResetL0 = 0x00000002,
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be
    // extended from locality 1
    ExtendL1 = 0x00000003,
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be reset
    // by TPM2_PCR_Reset() from locality 1
    ResetL1 = 0x00000004,
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be
    // extended from locality 2
    ExtendL2 = 0x00000005,
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be reset
    // by TPM2_PCR_Reset() from locality 2
    ResetL2 = 0x00000006,
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be
    // extended from locality 3
    ExtendL3 = 0x00000007,
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be reset
    // by TPM2_PCR_Reset() from locality 3
    ResetL3 = 0x00000008,
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be
    // extended from locality 4
    ExtendL4 = 0x00000009,
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be reset
    // by TPM2_PCR_Reset() from locality 4
    ResetL4 = 0x0000000A,
    // a SET bit in the TPMS_PCR_SELECT indicates that modifications to this
    // PCR (reset or Extend) will not increment the pcrUpdateCounter
    NoIncrement = 0x00000011,
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR is reset by a
    // D-RTM event
    DRTMRest = 0x00000012,
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR is controlled
    // by policy
    Policy = 0x00000013,
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR is controlled
    // by an authorization value
    Auth = 0x00000014,
}
