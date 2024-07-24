// =============================================================================
// COMMON USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;

// =============================================================================
// TYPES
// =============================================================================

#[derive(Clone, Copy, PartialEq, Default, Debug, Marshalable)]
pub struct TpmsAlgProperty {
    pub alg: TPM2AlgID,
    pub alg_properties: TpmaAlgorithm,
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsAsymParms {
    pub symmetric: TpmtSymDefObject,
    pub scheme: TpmtAsymScheme,
}

#[derive(Clone, Copy, PartialEq)]
pub struct TpmsAttest {
    pub magic: TPM2Generated,
    pub qualified_signer: Tpm2bName,
    pub extra_data: Tpm2bData,
    pub clock_info: TpmsClockInfo,
    pub firmware_version: u64,
    pub attested: TpmuAttest,
}
// Custom overload of Marshalable, because the selector for attested is {un}marshaled separate from the field.
// TODO: We definitely don't need these. We can make the derive macro smarter.
impl Marshalable for TpmsAttest {
    fn try_marshal(&self, buffer: &mut [u8]) -> TssTspResult<usize> {
        let mut written = 0;
        written += self.magic.try_marshal(&mut buffer[written..])?;
        written += self
            .attested
            .discriminant()
            .try_marshal(&mut buffer[written..])?;
        written += self.qualified_signer.try_marshal(&mut buffer[written..])?;
        written += self.extra_data.try_marshal(&mut buffer[written..])?;
        written += self.clock_info.try_marshal(&mut buffer[written..])?;
        written += self.firmware_version.try_marshal(&mut buffer[written..])?;
        written += self.attested.try_marshal_variant(&mut buffer[written..])?;
        Ok(written)
    }

    fn try_unmarshal(buffer: &mut UnmarshalBuf) -> TssTspResult<Self> {
        let magic = TPM2Generated::try_unmarshal(buffer)?;
        let selector = u16::try_unmarshal(buffer)?;
        Ok(TpmsAttest {
            magic,
            qualified_signer: Tpm2bName::try_unmarshal(buffer)?,
            extra_data: Tpm2bData::try_unmarshal(buffer)?,
            clock_info: TpmsClockInfo::try_unmarshal(buffer)?,
            firmware_version: u64::try_unmarshal(buffer)?,
            attested: TpmuAttest::try_unmarshal_variant(selector, buffer)?,
        })
    }
}

#[derive(Clone, Copy, Default, PartialEq, Debug, Marshalable)]
pub struct TpmsAuthCommand {
    pub session_handle: TpmiShAuthSession,
    pub nonce: Tpm2bNonce,
    pub session_attributes: TpmaSession,
    pub hmac: Tpm2bAuth,
}

#[derive(Clone, Copy, Default, PartialEq, Debug, Marshalable)]
pub struct TpmsAuthResponse {
    pub nonce: Tpm2bNonce,
    pub session_attributes: TpmaSession,
    pub hmac: Tpm2bData,
}

#[repr(C, u32)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub enum TpmsCapabilityData {
    Algorithms(TpmlAlgProperty) = TPM2Cap::Algs.0,
    Handles(TpmlHandle) = TPM2Cap::Handles.0,
    Command(TpmlCca) = TPM2Cap::Commands.0,
    PpCommands(TpmlCc) = TPM2Cap::PPCommands.0,
    AuditCommands(TpmlCc) = TPM2Cap::AuditCommands.0,
    AssignedPcr(TpmlPcrSelection) = TPM2Cap::PCRs.0,
    TpmProperties(TpmlTaggedTpmProperty) = TPM2Cap::TPMProperties.0,
    PcrProperties(TpmlTaggedPcrProperty) = TPM2Cap::PCRProperties.0,
    EccCurves(TpmlEccCurve) = TPM2Cap::ECCCurves.0,
    AuthPolicies(TpmlTaggedPolicy) = TPM2Cap::AuthPolicies.0,
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsCertifyInfo {
    pub name: Tpm2bName,
    pub qualified_name: Tpm2bName,
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsClockInfo {
    pub clock: u64,
    pub reset_count: u32,
    pub restart_count: u32,
    pub safe: TpmiYesNo,
}

#[derive(Clone, Copy, PartialEq, Marshalable)]
pub struct TpmsCommandAuditInfo {
    pub audit_counter: u64,
    pub digest_alg: u16,
    pub audit_digest: Tpm2bDigest,
    pub command_digest: Tpm2bDigest,
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsContextData {
    pub integrity: Tpm2bDigest,
    pub encrypted: Tpm2bContextSensitive,
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsCreationData {
    pub pcr_select: TpmlPcrSelection,
    pub pcr_digest: Tpm2bDigest,
    pub locality: TpmaLocality,
    pub parent_name_alg: TPM2AlgID,
    pub parent_name: Tpm2bName,
    pub parent_qualified_name: Tpm2bName,
    pub outside_info: Tpm2bData,
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsCreationInfo {
    pub object_name: Tpm2bName,
    pub creation_hash: Tpm2bDigest,
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsDerive {
    pub label: Tpm2bLabel,
    pub context: Tpm2bLabel,
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsEccParms {
    pub symmetric: TpmtSymDefObject,
    pub scheme: TpmtEccScheme,
    pub curve_id: TpmiEccCurve,
    pub kdf: TpmtKdfScheme,
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsEccPoint {
    pub x: Tpm2bEccParameter,
    pub y: Tpm2bEccParameter,
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsEmpty;

// TODO: This should probably be a newtype.
pub type TpmsEncSchemeOaep = TpmsSchemeHash;

// TODO: This should probably be a newtype.
pub type TpmsEncSchemeRsaes = TpmsEmpty;

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsIdObject {
    pub integrity_hmac: Tpm2bDigest,
    pub enc_identity: Tpm2bDigest,
}

// TODO: This should probably be a newtype.
pub type TpmsKeySchemeEcdh = TpmsSchemeHash;

// TODO: This should probably be a newtype.
pub type TpmsKeySchemeEcmqv = TpmsSchemeHash;

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsKeyedHashParms {
    pub scheme: TpmtKeyedHashScheme,
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsNvCertifyInfo {
    pub index_name: Tpm2bName,
    pub offset: u16,
    pub nv_contents: Tpm2bMaxNvBuffer,
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsNvPublic {
    pub nv_index: TpmiRhNvIndex,
    pub name_alg: TpmiAlgHash,
    pub attributes: TpmaNv,
    pub auth_policy: Tpm2bDigest,
    pub data_size: u16,
}

#[derive(Clone, Copy, PartialEq, Debug, Default, Marshalable)]
pub struct TpmsPcrSelection {
    pub hash: TpmiAlgHash,
    pub sizeof_select: u8,
    #[marshal(length=sizeof_select)]
    pub pcr_select: [u8; TPM2_PCR_SELECT_MAX as usize],
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsQuoteInfo {
    pub pcr_select: TpmlPcrSelection,
    pub pcr_digest: Tpm2bDigest,
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsRsaParms {
    pub symmetric: TpmtSymDefObject,
    pub scheme: TpmtRsaScheme,
    pub key_bits: TpmiRsaKeyBits,
    pub exponent: u32,
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsSchemeHash {
    pub hash_alg: TpmiAlgHash,
}

// TODO: This should probably be a newtype.
pub type TpmsSchemeHmac = TpmsSchemeHash;

// TODO: This should probably be a newtype.
pub type TpmsSchemeKdf1Sp800_56a = TpmsSchemeHash;

// TODO: This should probably be a newtype.
pub type TpmsSchemeKdf1Sp800_108 = TpmsSchemeHash;

// TODO: This should probably be a newtype.
pub type TpmsSchemeKdf2 = TpmsSchemeHash;

// TODO: This should probably be a newtype.
pub type TpmsSchemeMgf1 = TpmsSchemeHash;

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsSchemeXor {
    pub hash_alg: TpmiAlgHash,
    pub kdf: TpmiAlgKdf,
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsSensitiveCreate {
    pub user_auth: Tpm2bAuth,
    pub data: Tpm2bSensitiveData,
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsSessionAuditInfo {
    pub exclusive_session: TpmiYesNo,
    pub session_digest: Tpm2bDigest,
}

// TODO: This should probably be a newtype.
pub type TpmsSigSchemeEcdaa = TpmsSchemeHash;

// TODO: This should probably be a newtype.
pub type TpmsSigSchemeEcdsa = TpmsSchemeHash;

// TODO: This should probably be a newtype.
pub type TpmsSigSchemeEcschnorr = TpmsSchemeHash;

// TODO: This should probably be a newtype.
pub type TpmsSigSchemeRsapss = TpmsSchemeHash;

// TODO: This should probably be a newtype.
pub type TpmsSigSchemeRsassa = TpmsSchemeHash;

// TODO: This should probably be a newtype.
pub type TpmsSigSchemeSm2 = TpmsSchemeHash;

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsSymCipherParms {
    pub sym: TpmtSymDefObject,
}

#[derive(Clone, Copy, PartialEq, Default, Debug, Marshalable)]
pub struct TpmsTaggedPcrSelect {
    tag: TPM2PTPCR,
    size_of_select: u8,
    #[marshal(length=size_of_select)]
    pcr_select: [u8; TPM2_PCR_SELECT_MAX as usize],
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable, Default)]
pub struct TpmsTaggedPolicy {
    handle: TPM2Handle,
    policy_hash: TpmtHa,
}

#[derive(Clone, Copy, PartialEq, Default, Debug, Marshalable)]
pub struct TpmsTaggedProperty {
    pub property: TPM2PT,
    pub value: u32,
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsTimeAttestInfo {
    pub time: TpmsTimeInfo,
    pub firmware_version: u64,
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmsTimeInfo {
    pub time: u64,
    pub clock_info: TpmsClockInfo,
}
