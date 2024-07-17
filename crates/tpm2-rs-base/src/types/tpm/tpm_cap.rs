use super::*;

// TPM2Cap represents a TPM_CAP.
// See definition in Part 2: Structures, section 6.12
#[open_enum]
#[repr(u32)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, Default, Marshalable)]
pub enum TPM2Cap {
    Algs = 0x00000000,
    Handles = 0x00000001,
    Commands = 0x00000002,
    PPCommands = 0x00000003,
    AuditCommands = 0x00000004,
    PCRs = 0x00000005,
    TPMProperties = 0x00000006,
    PCRProperties = 0x00000007,
    ECCCurves = 0x00000008,
    AuthPolicies = 0x00000009,
    ACT = 0x0000000A,
}
