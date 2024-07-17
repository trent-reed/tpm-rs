use super::*;

/// TPM2Generated represents a TPM_GENERATED.
/// See definition in Part 2: Structures, section 6.2.
#[open_enum]
#[repr(u32)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, Default, Marshalable)]
pub enum TPM2Generated {
    VALUE = 0xFF544347,
}
