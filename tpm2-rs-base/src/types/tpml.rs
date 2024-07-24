// =============================================================================
// COMMON USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;

// =============================================================================
// MACROS
// =============================================================================

// -----------------------------------------------------------------------------
// TODO: See if we really need this macro.
macro_rules! impl_tpml {
  ($T:ty,  $ListField:ident, $ListType:ty, $ListCapacity:ident) => {
      // Implement Default for the type. This cannot usually be derived, because $ListCapacity is too large.
      impl Default for $T {
          fn default() -> Self {
              Self {
                  count: 0,
                  $ListField: [<$ListType>::default(); $ListCapacity as usize],
              }
          }
      }

      impl $T {
          pub fn new(elements: &[$ListType]) -> TssTspResult<$T> {
              if elements.len() > $ListCapacity as usize {
                  return Err(TssTspError::new(TssErrorCode::InternalError));
              }
              let mut x = Self::default();
              x.count = elements.len() as u32;
              x.$ListField[..elements.len()].copy_from_slice(elements);
              Ok(x)
          }

          pub fn add(&mut self, element: &$ListType) -> TssTspResult<()> {
              if self.count() >= self.$ListField.len() {
                return Err(TssTspError::new(TssErrorCode::InternalError));
              }
              self.$ListField[self.count()] = *element;
              self.count += 1;
              Ok(())
          }

          pub fn count(&self) -> usize {
              self.count as usize
          }

          pub fn $ListField(&self) -> &[$ListType] {
              &self.$ListField[..min(self.count(), $ListCapacity as usize)]
          }
      }
  };
}

// =============================================================================
// TYPES
// =============================================================================

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmlAlgProperty {
    count: u32,
    #[marshal(length=count)]
    alg_properties: [TpmsAlgProperty; TPM2_MAX_CAP_ALGS],
}
impl_tpml! {TpmlAlgProperty, alg_properties, TpmsAlgProperty, TPM2_MAX_CAP_ALGS}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmlCc {
    count: u32,
    #[marshal(length=count)]
    command_codes: [TPM2CC; TPM2_MAX_CAP_CC],
}
impl_tpml! {TpmlCc, command_codes, TPM2CC, TPM2_MAX_CAP_CC}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmlCca {
    count: u32,
    #[marshal(length=count)]
    command_attributes: [TpmaCc; TPM2_MAX_CAP_CC],
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmlDigest {
    count: u32,
    #[marshal(length=count)]
    digests: [Tpm2bDigest; TPML_DIGEST_MAX_DIGESTS],
}
impl_tpml! {TpmlDigest, digests, Tpm2bDigest, TPML_DIGEST_MAX_DIGESTS}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmlEccCurve {
    count: u32,
    #[marshal(length=count)]
    ecc_curves: [TPM2ECCCurve; TPM2_MAX_ECC_CURVES],
}
impl_tpml! {TpmlEccCurve, ecc_curves, TPM2ECCCurve, TPM2_MAX_ECC_CURVES}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmlHandle {
    count: u32,
    #[marshal(length=count)]
    handle: [TPM2Handle; TPM2_MAX_CAP_HANDLES],
}
impl_tpml! {TpmlHandle, handle, TPM2Handle, TPM2_MAX_CAP_HANDLES}


#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmlPcrSelection {
    count: u32,
    #[marshal(length=count)]
    pcr_selections: [TpmsPcrSelection; TPM2_NUM_PCR_BANKS as usize],
}
impl_tpml! {TpmlPcrSelection, pcr_selections, TpmsPcrSelection, TPM2_NUM_PCR_BANKS}


#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmlTaggedPcrProperty {
    count: u32,
    #[marshal(length=count)]
    pcr_property: [TpmsTaggedPcrSelect; TPM2_MAX_PCR_PROPERTIES],
}
impl_tpml! {TpmlTaggedPcrProperty, pcr_property, TpmsTaggedPcrSelect, TPM2_MAX_PCR_PROPERTIES}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmlTaggedPolicy {
    count: u32,
    #[marshal(length=count)]
    policies: [TpmsTaggedPolicy; TPM2_MAX_TAGGED_POLICIES],
}

#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub struct TpmlTaggedTpmProperty {
    count: u32,
    #[marshal(length=count)]
    tpm_property: [TpmsTaggedProperty; TPM2_MAX_TPM_PROPERTIES],
}
impl_tpml! {TpmlTaggedTpmProperty, tpm_property, TpmsTaggedProperty, TPM2_MAX_TPM_PROPERTIES}
