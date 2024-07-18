use super::*;

#[repr(C, u16)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub enum TpmuSensitiveComposite {
    Rsa(Tpm2bPrivateKeyRsa) = TPM2AlgID::RSA.0,
    Ecc(Tpm2bEccParameter) = TPM2AlgID::ECC.0,
    Bits(Tpm2bSensitiveData) = TPM2AlgID::KeyedHash.0,
    Sym(Tpm2bSymKey) = TPM2AlgID::SymCipher.0,
    /* For size purposes only */
    Any(Tpm2bPrivateVendorSpecific) = TPM2AlgID::Null.0,
}
