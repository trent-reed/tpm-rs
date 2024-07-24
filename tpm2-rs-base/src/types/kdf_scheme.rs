use super::*;

#[repr(C, u16)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub enum TpmtKdfScheme {
    Mgf1(TpmsSchemeMgf1) = TPM2AlgID::MGF1.0,
    Kdf1Sp800_56a(TpmsSchemeKdf1Sp800_56a) = TPM2AlgID::KDF1SP80056A.0,
    Kdf2(TpmsSchemeKdf2) = TPM2AlgID::KDF2.0,
    Kdf1Sp800_108(TpmsSchemeKdf1Sp800_108) = TPM2AlgID::KDF1SP800108.0,
    Null(TpmsEmpty) = TPM2AlgID::Null.0,
}
