use super::*;

/// TpmiAlgHash represents all of the hash algorithms (TPMI_ALG_HASH).
/// See definition in Part 2: Structures, section 9.27.
#[open_enum]
#[repr(u16)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, PartialEq, Default, Marshalable)]
pub enum TpmiAlgHash {
    SHA1 = TPM2AlgID::SHA1.0,
    SHA256  = TPM2AlgID::SHA256.0,
    SHA384   = TPM2AlgID::SHA384.0,
    SHA512 = TPM2AlgID::SHA512.0,
    SM3256 = TPM2AlgID::SM3256.0,
    SHA3256 = TPM2AlgID::SHA3256.0,
    SHA3384 = TPM2AlgID::SHA3384.0,
    SHA3512 = TPM2AlgID::SHA3512.0,
}
