use super::*;

/// TpmiStCommandTag defines the command tags (TPMI_ST_COMMAND_TAG).
/// See definition in Part 2: Structures, section 9.35.
#[open_enum]
#[repr(u16)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, PartialEq, Default, Marshalable)]
pub enum TpmiStCommandTag{
    NoSessions = TPM2ST::NoSessions.0,
    Sessions = TPM2ST::Sessions.0,
}
