use super::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TpmtSensitive {
    pub auth_value: Tpm2bAuth,
    pub seed_value: Tpm2bDigest,
    pub sensitive: TpmuSensitiveComposite,
}
// Custom overload of Marshalable, because the selector for sensitive is {un}marshaled first.
impl Marshalable for TpmtSensitive {
    fn try_marshal(&self, buffer: &mut [u8]) -> TpmResult<usize> {
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

    fn try_unmarshal(buffer: &mut UnmarshalBuf) -> TpmResult<Self> {
        let selector = u16::try_unmarshal(buffer)?;
        Ok(TpmtSensitive {
            auth_value: Tpm2bAuth::try_unmarshal(buffer)?,
            seed_value: Tpm2bDigest::try_unmarshal(buffer)?,
            // TODO: This is not great, it's us reaching into the guts of marshal.
            sensitive: TpmuSensitiveComposite::try_unmarshal_variant(selector, buffer)?,
        })
    }
}
