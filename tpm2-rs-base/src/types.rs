// =============================================================================
// COMMON USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;

// =============================================================================
// MACROS
//   These need to come first based on how macro rules work. You must define the
//   macro before the usage in the module file.
//
//   TODO: We can probably get rid of these with some helper subtypes.
// =============================================================================

// -----------------------------------------------------------------------------
// TODO: I highly suspect this can go away... But I need to keep refactoring.
macro_rules! impl_try_marshalable_tpm2b_simple {
    ($T:ty, $F:ident) => {
        impl crate::types::Tpm2bSimple for $T {
            const MAX_BUFFER_SIZE: usize = size_of::<$T>() - size_of::<u16>();

            // TODO: Used in Marshalable, but is this all really necessary?
            //       Also used in client a little.
            fn get_size(&self) -> u16 {
                self.size
            }

            // TODO: I have no idea what this function is for...
            fn get_buffer(&self) -> &[u8] {
                &self.$F[0..self.get_size() as usize]
            }

            // TODO: I have no idea what this function is for...
            //       I assume it's to check bounds on buffers or something,
            //       It's only used in tests and the tests don't make it clearer.
            //       Maybe old code that used to be used in marshaling and can be
            //       removed now?
            fn from_bytes(buffer: &[u8]) -> TssTspResult<Self> {
                // Overflow check
                if buffer.len() > core::cmp::min(u16::MAX as usize, Self::MAX_BUFFER_SIZE) {
                    return Err(TssTspError::new(TssErrorCode::InternalError));
                }

                let mut dest: Self = Self {
                    size: buffer.len() as u16,
                    $F: [0; Self::MAX_BUFFER_SIZE],
                };
                dest.$F[..buffer.len()].copy_from_slice(buffer);
                Ok(dest)
            }
        }

        impl Default for $T {
            fn default() -> Self {
                use crate::types::Tpm2bSimple;
                Self {
                    size: 0,
                    $F: [0; Self::MAX_BUFFER_SIZE],
                }
            }
        }

        impl Marshalable for $T {
            fn try_unmarshal(buffer: &mut UnmarshalBuf) -> TssTspResult<Self> {
                use crate::types::Tpm2bSimple;
                let got_size = u16::try_unmarshal(buffer)?;
                // Ensure the buffer is large enough to fullfill the size indicated
                // TODO: Probably we need to clean up UnmarshalBuf.
                let sized_buffer = buffer.get(got_size as usize);
                if !sized_buffer.is_some() {
                    return Err(TssTspError::new(TssErrorCode::InternalError));
                }

                let mut dest: Self = Self {
                    size: got_size,
                    $F: [0; Self::MAX_BUFFER_SIZE],
                };

                // Make sure the size indicated isn't too large for the types buffer
                if sized_buffer.unwrap().len() > dest.$F.len() {
                    return Err(TssTspError::new(TssErrorCode::InternalError));
                }
                dest.$F[..got_size.into()].copy_from_slice(&sized_buffer.unwrap());

                Ok(dest)
            }

            fn try_marshal(&self, buffer: &mut [u8]) -> TssTspResult<usize> {
                use crate::types::Tpm2bSimple;
                let used = self.size.try_marshal(buffer)?;
                let (_, rest) = buffer.split_at_mut(used);
                let buffer_marsh = self.get_size() as usize;
                if buffer_marsh > (core::cmp::max(Self::MAX_BUFFER_SIZE, rest.len())) {
                    return Err(TssTspError::new(TssErrorCode::InternalError));
                }
                rest[..buffer_marsh].copy_from_slice(&self.$F[..buffer_marsh]);
                Ok(used + buffer_marsh)
            }
        }
    };
}

// -----------------------------------------------------------------------------
macro_rules! impl_try_marshalable_tpm2b_struct {
    ($T:ty, $StructType:ty, $F:ident) => {
        impl crate::types::Tpm2bStruct for $T {
            type StructType = $StructType;

            fn from_struct(val: &Self::StructType) -> TssTspResult<Self> {
                let mut x = Self::default();
                x.size = val.try_marshal(&mut x.$F)? as u16;
                Ok(x)
            }

            fn to_struct(&self) -> TssTspResult<Self::StructType> {
                use crate::types::Tpm2bSimple;
                let mut buf = UnmarshalBuf::new(&self.$F[0..self.get_size() as usize]);
                Self::StructType::try_unmarshal(&mut buf)
            }
        }
    };
}

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
// COMMON FUNCTIONS FOR SUBMODULES
// =============================================================================

/// Returns an attribute field built by applying the mask/shift to the value.
const fn new_attribute_field(value: u32, mask: u32, shift: u32) -> u32 {
    (value << shift) & mask
}

/// Returns the attribute field retrieved from the value with the mask/shift.
const fn get_attribute_field(value: u32, mask: u32, shift: u32) -> u32 {
    (value & mask) >> shift
}

/// Sets the attribute field defined by mask/shift in the value to the field value, and returns the result.
const fn set_attribute_field(value: u32, field_value: u32, mask: u32, shift: u32) -> u32 {
    (value & !mask) | new_attribute_field(field_value, mask, shift)
}

// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
mod aes_key_bits;
pub use aes_key_bits::*;
mod alg_asym_scheme;
pub use alg_asym_scheme::*;
mod alg_ecc_scheme;
pub use alg_ecc_scheme::*;
mod alg_hash;
pub use alg_hash::*;
mod alg_id;
pub use alg_id::*;
mod alg_kdf;
pub use alg_kdf::*;
mod alg_keyedhash_scheme;
pub use alg_keyedhash_scheme::*;
mod alg_property;
pub use alg_property::*;
mod alg_public;
pub use alg_public::*;
mod alg_rsa_scheme;
pub use alg_rsa_scheme::*;
mod alg_sym_mode;
pub use alg_sym_mode::*;
mod alg_sym_object;
pub use alg_sym_object::*;
mod algorithm;
pub use algorithm::*;
mod asym_parms;
pub use asym_parms::*;
mod asym_scheme;
pub use asym_scheme::*;
mod attest;
pub use attest::*;
mod auth_command;
pub use auth_command::*;
mod auth_response;
pub use auth_response::*;
mod auth;
pub use auth::*;
mod camellia_key_bits;
pub use camellia_key_bits::*;
mod cap;
pub use cap::*;
mod capability_data;
pub use capability_data::*;
mod cc;
pub use cc::*;
mod cca;
pub use cca::*;
mod certify_info;
pub use certify_info::*;
mod clock_info;
pub use clock_info::*;
mod command_audit_info;
pub use command_audit_info::*;
mod context_data;
pub use context_data::*;
mod context_sensitive;
pub use context_sensitive::*;
mod creation_data;
pub use creation_data::*;
mod creation_info;
pub use creation_info::*;
mod data;
pub use data::*;
mod derive;
pub use derive::*;
mod digest;
pub use digest::*;
mod ecc_curve;
pub use ecc_curve::*;
mod ecc_parameter;
pub use ecc_parameter::*;
mod ecc_parms;
pub use ecc_parms::*;
mod ecc_point;
pub use ecc_point::*;
mod ecc_scheme;
pub use ecc_scheme::*;
mod empty;
pub use empty::*;
mod enc_scheme_oaep;
pub use enc_scheme_oaep::*;
mod enc_scheme_rsaes;
pub use enc_scheme_rsaes::*;
mod encrypted_secret;
pub use encrypted_secret::*;
mod eo;
pub use eo::*;
mod event;
pub use event::*;
mod generated;
pub use generated::*;
mod ha;
pub use ha::*;
mod handle;
pub use handle::*;
mod hc;
pub use hc::*;
mod ht;
pub use ht::*;
mod id_object;
pub use id_object::*;
mod iv;
pub use iv::*;
mod kdf_scheme;
pub use kdf_scheme::*;
mod key_scheme_ecdh;
pub use key_scheme_ecdh::*;
mod key_scheme_ecmqv;
pub use key_scheme_ecmqv::*;
mod keyed_hash_params;
pub use keyed_hash_params::*;
mod keyed_hash_scheme;
pub use keyed_hash_scheme::*;
mod label;
pub use label::*;
mod locality;
pub use locality::*;
mod max_buffer;
pub use max_buffer::*;
mod max_cap_buffer;
pub use max_cap_buffer::*;
mod max_nv_buffer;
pub use max_nv_buffer::*;
mod name;
pub use name::*;
mod nonce;
pub use nonce::*;
mod nt;
pub use nt::*;
mod nv_certify_info;
pub use nv_certify_info::*;
mod nv_public;
pub use nv_public::*;
mod nv;
pub use nv::*;
mod object;
pub use object::*;
mod operand;
pub use operand::*;
mod pcr_selection;
pub use pcr_selection::*;
mod private_key_rsa;
pub use private_key_rsa::*;
mod private_vendor_specific;
pub use private_vendor_specific::*;
mod private;
pub use private::*;
mod pt_pcr;
pub use pt_pcr::*;
mod pt;
pub use pt::*;
mod public_id;
pub use public_id::*;
mod public_key_rsa;
pub use public_key_rsa::*;
mod public;
pub use public::*;
mod quote_info;
pub use quote_info::*;
mod rc;
pub use rc::*;
mod rh_hv_index;
pub use rh_hv_index::*;
mod rsa_key_bits;
pub use rsa_key_bits::*;
mod rsa_parms;
pub use rsa_parms::*;
mod rsa_scheme;
pub use rsa_scheme::*;
mod scheme_hash;
pub use scheme_hash::*;
mod scheme_hmac;
pub use scheme_hmac::*;
mod scheme_kdf1_sp800_56a;
pub use scheme_kdf1_sp800_56a::*;
mod scheme_kdf1_sp800_108;
pub use scheme_kdf1_sp800_108::*;
mod scheme_kdf2;
pub use scheme_kdf2::*;
mod scheme_mgf1;
pub use scheme_mgf1::*;
mod scheme_xor;
pub use scheme_xor::*;
mod se;
pub use se::*;
mod sensitive_composite;
pub use sensitive_composite::*;
mod sensitive_create;
pub use sensitive_create::*;
mod sensitive_data;
pub use sensitive_data::*;
mod sensitive;
pub use sensitive::*;
mod session_audit_info;
pub use session_audit_info::*;
mod session;
pub use session::*;
mod sh_auth_session;
pub use sh_auth_session::*;
mod sig_scheme_ecdaa;
pub use sig_scheme_ecdaa::*;
mod sig_scheme_ecdsa;
pub use sig_scheme_ecdsa::*;
mod sig_scheme_ecschnorr;
pub use sig_scheme_ecschnorr::*;
mod sig_scheme_rsapss;
pub use sig_scheme_rsapss::*;
mod sig_scheme_rsassa;
pub use sig_scheme_rsassa::*;
mod sig_scheme_sm2;
pub use sig_scheme_sm2::*;
mod simple;
pub use simple::*;
mod sm4_key_bits;
pub use sm4_key_bits::*;
mod st_attest;
pub use st_attest::*;
mod st_command_tag;
pub use st_command_tag::*;
mod st;
pub use st::*;
mod r#struct;
pub use r#struct::*;
mod su;
pub use su::*;
mod sym_cipher_parms;
pub use sym_cipher_parms::*;
mod sym_def_object;
pub use sym_def_object::*;
mod sym_key;
pub use sym_key::*;
mod tagged_pcr_property;
pub use tagged_pcr_property::*;
mod tagged_pcr_select;
pub use tagged_pcr_select::*;
mod tagged_policy;
pub use tagged_policy::*;
mod tagged_property;
pub use tagged_property::*;
mod tagged_tpm_property;
pub use tagged_tpm_property::*;
mod template;
pub use template::*;
mod time_attest_info;
pub use time_attest_info::*;
mod time_info;
pub use time_info::*;
mod yes_no;
pub use yes_no::*;
