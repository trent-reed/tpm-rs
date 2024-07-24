use super::*;

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
