// =============================================================================
// COMMON USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;

// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
mod public;
pub use public::*;
mod sensitive;
pub use sensitive::*;

// =============================================================================
// TYPES
// =============================================================================

#[repr(C, u16)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub enum TpmtAsymScheme {
    Ecdh(TpmsKeySchemeEcdh) = TPM2AlgID::ECDH.0,
    Ecmqv(TpmsKeySchemeEcmqv) = TPM2AlgID::ECMQV.0,
    Sm2(TpmsSigSchemeSm2) = TPM2AlgID::SM2.0,
    Rsapss(TpmsSigSchemeRsapss) = TPM2AlgID::RSAPSS.0,
    Rsassa(TpmsSigSchemeRsassa) = TPM2AlgID::RSASSA.0,
    Ecdsa(TpmsSigSchemeEcdsa) = TPM2AlgID::ECDSA.0,
    Ecdaa(TpmsSigSchemeEcdaa) = TPM2AlgID::ECDAA.0,
    Ecschnorr(TpmsSigSchemeEcschnorr) = TPM2AlgID::ECSchnorr.0,
    Rsaes(TpmsEncSchemeRsaes) = TPM2AlgID::RSAES.0,
    Oaep(TpmsEncSchemeOaep) = TPM2AlgID::OAEP.0,
    Null(TpmsEmpty) = TPM2AlgID::Null.0,
}

#[repr(C, u16)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub enum TpmtEccScheme {
    Rsapss(TpmsSigSchemeRsapss) = TPM2AlgID::RSAPSS.0,
    Rsassa(TpmsSigSchemeRsassa) = TPM2AlgID::RSASSA.0,
    Ecdsa(TpmsSigSchemeEcdsa) = TPM2AlgID::ECDSA.0,
    Ecdaa(TpmsSigSchemeEcdaa) = TPM2AlgID::ECDAA.0,
    Sm2(TpmsSigSchemeSm2) = TPM2AlgID::SM2.0,
    Ecschnorr(TpmsSigSchemeEcschnorr) = TPM2AlgID::ECSchnorr.0,
    Ecdh(TpmsKeySchemeEcdh) = TPM2AlgID::ECDH.0,
    Ecmqv(TpmsKeySchemeEcmqv) = TPM2AlgID::ECMQV.0,
    Null(TpmsEmpty) = TPM2AlgID::Null.0,
}

#[repr(C, u16)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable, UnionSize)]
pub enum TpmtHa {
    Sha1([u8; TPM2_SHA_DIGEST_SIZE as usize]) = TPM2AlgID::SHA1.0,
    Sha256([u8; TPM2_SHA256_DIGEST_SIZE as usize]) = TPM2AlgID::SHA256.0,
    Sha384([u8; TPM2_SHA384_DIGEST_SIZE as usize]) = TPM2AlgID::SHA384.0,
    Sha512([u8; TPM2_SHA512_DIGEST_SIZE as usize]) = TPM2AlgID::SHA512.0,
    Sm3_256([u8; TPM2_SM3_256_DIGEST_SIZE as usize]) = TPM2AlgID::SM3256.0,
}
impl Default for TpmtHa {
    fn default() -> Self {
        TpmtHa::Sha1([0; TPM2_SHA1_DIGEST_SIZE as usize])
    }
}

#[repr(C, u16)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub enum TpmtKdfScheme {
    Mgf1(TpmsSchemeMgf1) = TPM2AlgID::MGF1.0,
    Kdf1Sp800_56a(TpmsSchemeKdf1Sp800_56a) = TPM2AlgID::KDF1SP80056A.0,
    Kdf2(TpmsSchemeKdf2) = TPM2AlgID::KDF2.0,
    Kdf1Sp800_108(TpmsSchemeKdf1Sp800_108) = TPM2AlgID::KDF1SP800108.0,
    Null(TpmsEmpty) = TPM2AlgID::Null.0,
}

#[repr(C, u16)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub enum TpmtKeyedHashScheme {
    Hmac(TpmsSchemeHmac) = TPM2AlgID::HMAC.0,
    ExclusiveOr(TpmsSchemeXor) = TPM2AlgID::XOR.0,
    Null(TpmsEmpty) = TPM2AlgID::Null.0,
}

#[repr(C, u16)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub enum TpmtRsaScheme {
    Rsapss(TpmsSigSchemeRsapss) = TPM2AlgID::RSAPSS.0,
    Rsassa(TpmsSigSchemeRsassa) = TPM2AlgID::RSASSA.0,
    Ecdsa(TpmsSigSchemeEcdsa) = TPM2AlgID::ECDSA.0,
    Ecdaa(TpmsSigSchemeEcdaa) = TPM2AlgID::ECDAA.0,
    Sm2(TpmsSigSchemeSm2) = TPM2AlgID::SM2.0,
    Ecschnorr(TpmsSigSchemeEcschnorr) = TPM2AlgID::ECSchnorr.0,
    Rsaes(TpmsEncSchemeRsaes) = TPM2AlgID::RSAES.0,
    Oaep(TpmsEncSchemeOaep) = TPM2AlgID::OAEP.0,
    Null(TpmsEmpty) = TPM2AlgID::Null.0,
}

// TODO: I'd rather have helper structs.
#[repr(C, u16)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub enum TpmtSymDefObject {
    Aes(TpmiAesKeyBits, TpmiAlgSymMode) = TPM2AlgID::AES.0,
    Sm4(TpmiSm4KeyBits, TpmiAlgSymMode) = TPM2AlgID::SM4.0,
    Camellia(TpmiCamelliaKeyBits, TpmiAlgSymMode) = TPM2AlgID::Camellia.0,
    ExclusiveOr(TpmiAlgHash, TpmsEmpty) = TPM2AlgID::XOR.0,
    Null(TpmsEmpty, TpmsEmpty) = TPM2AlgID::Null.0,
}
