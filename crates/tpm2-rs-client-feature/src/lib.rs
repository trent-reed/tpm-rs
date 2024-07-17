// TODO: I haven't looked at this in-depth yet.

#![forbid(unsafe_code)]
use tpm2_rs_base::commands::*;
use tpm2_rs_base::{TPM2Cap, TPM2PT, TpmsCapabilityData};
use tpm2_rs_errors::*;
use tpm2_rs_client::*;

// # Feature Client
//
// The feature client provides higher-level abstractions than the base TPM client.

// Gets the TPM manufacturer ID.
pub fn get_manufacturer_id(client: &mut TpmClient<impl TpmConnection>) -> TssResult<u32>
{
    const CMD: GetCapabilityCmd = GetCapabilityCmd {
        capability: TPM2Cap::TPMProperties,
        property: TPM2PT::Manufacturer,
        property_count: 1,
    };
    let resp = client.get_capability(&CMD)?;
    if let TpmsCapabilityData::TpmProperties(prop) = resp.capability_data {
        if prop.count() == 1 {
            Ok(prop.tpm_property()[0].value)
        } else {
            Err(TPM_RC_SIZE.into())
        }
    } else {
        Err(TPM_RC_SELECTOR.into())
    }
}

#[cfg(test)]
mod tests;
