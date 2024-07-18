use super::*;

/// TpmiYesNo is used in place of a boolean.
/// See TPMI_YES_NO definition in Part 2: Structures, section 9.2.
#[open_enum]
#[repr(u8)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, PartialEq, Default, Marshalable)]
pub enum TpmiYesNo {
    NO = 0,
    YES = 1,
}
