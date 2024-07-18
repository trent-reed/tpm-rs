use super::*;

// TPM2ST represents a TPM_ST.
// See definition in Part 2: Structures, section 6.9.
#[open_enum]
#[repr(u16)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, Default, Marshalable)]
pub enum TPM2ST {
    RspCommand = 0x00C4,
    Null = 0x8000,
    NoSessions = 0x8001,
    Sessions = 0x8002,
    AttestNV = 0x8014,
    AttestCommandAudit = 0x8015,
    AttestSessionAudit = 0x8016,
    AttestCertify = 0x8017,
    AttestQuote = 0x8018,
    AttestTime = 0x8019,
    AttestCreation = 0x801A,
    AttestNVDigest = 0x801C,
    Creation = 0x8021,
    Verified = 0x8022,
    AuthSecret = 0x8023,
    HashCheck = 0x8024,
    AuthSigned = 0x8025,
    FuManifest = 0x8029,
}
