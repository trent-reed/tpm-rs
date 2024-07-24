// =============================================================================
// COMMON USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;

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

// TODO: This type is not defined by the standard.
// We should probably have a special naming convention.
// TODO: We probably should not have these unnamed structs.
#[repr(C, u16)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub enum PublicParmsAndId {
    KeyedHash(TpmsKeyedHashParms, Tpm2bDigest) = TPM2AlgID::KeyedHash.0,
    Sym(TpmsSymCipherParms, Tpm2bDigest) = TPM2AlgID::SymCipher.0,
    Rsa(TpmsRsaParms, Tpm2bPublicKeyRsa) = TPM2AlgID::RSA.0,
    Ecc(TpmsEccParms, TpmsEccPoint) = TPM2AlgID::ECC.0,
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TpmtPublic {
    pub name_alg: TpmiAlgHash,
    pub object_attributes: TpmaObject,
    pub auth_policy: Tpm2bDigest,
    // #[marshal(root_discriminant)]
    pub parms_and_id: PublicParmsAndId,
}
// Custom overload of Marshalable, because the selector for parms_and_id is {un}marshaled first.
// TODO: We definitely don't need these. We can make the derive macro smarter.
impl Marshalable for TpmtPublic {
    fn try_marshal(&self, buffer: &mut [u8]) -> TssTspResult<usize> {
        let mut written = 0;
        written += self
            .parms_and_id
            .discriminant()
            .try_marshal(&mut buffer[written..])?;
        written += self.name_alg.try_marshal(&mut buffer[written..])?;
        written += self.object_attributes.try_marshal(&mut buffer[written..])?;
        written += self.auth_policy.try_marshal(&mut buffer[written..])?;
        written += self
            .parms_and_id
            .try_marshal_variant(&mut buffer[written..])?;
        Ok(written)
    }
    fn try_unmarshal(buffer: &mut UnmarshalBuf) -> TssTspResult<Self> {
        let selector = u16::try_unmarshal(buffer)?;
        Ok(TpmtPublic {
            name_alg: TpmiAlgHash::try_unmarshal(buffer)?,
            object_attributes: TpmaObject::try_unmarshal(buffer)?,
            auth_policy: Tpm2bDigest::try_unmarshal(buffer)?,
            parms_and_id: PublicParmsAndId::try_unmarshal_variant(selector, buffer)?,
        })
    }
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

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TpmtSensitive {
    pub auth_value: Tpm2bAuth,
    pub seed_value: Tpm2bDigest,
    pub sensitive: TpmuSensitiveComposite,
}
// Custom overload of Marshalable, because the selector for sensitive is {un}marshaled first.
// TODO: We don't need this, we can make derive macro smarter.
impl Marshalable for TpmtSensitive {
    fn try_marshal(&self, buffer: &mut [u8]) -> TssTspResult<usize> {
        let mut written = 0;
        written += self
            .sensitive
            .discriminant()
            .try_marshal(&mut buffer[written..])?;
        written += self.auth_value.try_marshal(&mut buffer[written..])?;
        written += self.seed_value.try_marshal(&mut buffer[written..])?;
        written += self.sensitive.try_marshal_variant(&mut buffer[written..])?;
        Ok(written)
    }

    fn try_unmarshal(buffer: &mut UnmarshalBuf) -> TssTspResult<Self> {
        let selector = u16::try_unmarshal(buffer)?;
        Ok(TpmtSensitive {
            auth_value: Tpm2bAuth::try_unmarshal(buffer)?,
            seed_value: Tpm2bDigest::try_unmarshal(buffer)?,
            // TODO: This is not great, it's us reaching into the guts of marshal.
            sensitive: TpmuSensitiveComposite::try_unmarshal_variant(selector, buffer)?,
        })
    }
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
