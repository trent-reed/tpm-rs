use super::*;

#[repr(C, u16)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub enum TpmtKeyedHashScheme {
    Hmac(TpmsSchemeHmac) = TPM2AlgID::HMAC.0,
    ExclusiveOr(TpmsSchemeXor) = TPM2AlgID::XOR.0,
    Null(TpmsEmpty) = TPM2AlgID::Null.0,
}
