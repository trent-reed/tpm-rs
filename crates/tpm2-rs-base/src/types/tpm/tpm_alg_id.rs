use super::*;

// TPM2AlgID represents a TPM_ALG_ID.
// See definition in Part 2: Structures, section 6.3.
#[open_enum]
#[repr(u16)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, Default, Marshalable)]
pub enum TPM2AlgID {
    RSA = 0x0001,
    TDES = 0x0003,
    SHA1 = 0x0004,
    HMAC = 0x0005,
    AES = 0x0006,
    MGF1 = 0x0007,
    KeyedHash = 0x0008,
    XOR = 0x000A,
    SHA256 = 0x000B,
    SHA384 = 0x000C,
    SHA512 = 0x000D,
    Null = 0x0010,
    SM3256 = 0x0012,
    SM4 = 0x0013,
    RSASSA = 0x0014,
    RSAES = 0x0015,
    RSAPSS = 0x0016,
    OAEP = 0x0017,
    ECDSA = 0x0018,
    ECDH = 0x0019,
    ECDAA = 0x001A,
    SM2 = 0x001B,
    ECSchnorr = 0x001C,
    ECMQV = 0x001D,
    KDF1SP80056A = 0x0020,
    KDF2 = 0x0021,
    KDF1SP800108 = 0x0022,
    ECC = 0x0023,
    SymCipher = 0x0025,
    Camellia = 0x0026,
    SHA3256 = 0x0027,
    SHA3384 = 0x0028,
    SHA3512 = 0x0029,
    CMAC = 0x003F,
    CTR = 0x0040,
    OFB = 0x0041,
    CBC = 0x0042,
    CFB = 0x0043,
    ECB = 0x0044,
}
