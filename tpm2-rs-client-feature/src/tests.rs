use super::*;
use tpm2_rs_base::{TPM2AlgID, TPM2ST};
use tpm2_rs_marshal::Marshalable;
use tpm2_rs_base::{
    TpmaAlgorithm, TpmiYesNo, TpmlAlgProperty, TpmlTaggedTpmProperty, TpmsAlgProperty,
    TpmsTaggedProperty,
};

// TODO: Instead of having to redefine the mock, it might be nice to share the
// mock between different crates with a common `tpm2-rs-testonly` crate.
struct FakeTpmConnection {
    len: usize,
    response: [u8; RESP_BUFFER_SIZE],
}
impl Default for FakeTpmConnection {
    fn default() -> Self {
      FakeTpmConnection {
            len: 0,
            response: [0; RESP_BUFFER_SIZE],
        }
    }
}
impl TpmConnection for FakeTpmConnection {
    fn transact(&mut self, _: &[u8], response: &mut [u8]) -> TssResult<()> {
        let mut tx_header = RespHeader {
            tag: TPM2ST::NoSessions,
            size: 0,
            rc: 0,
        };
        let off = tx_header.try_marshal(response)?;
        let length = off + self.len;
        if length > response.len() {
            return Err(TPM_RC_SIZE.into());
        }
        response[off..length].copy_from_slice(&self.response[..self.len]);
        tx_header.size = length as u32;
        tx_header.try_marshal(response)?;
        Ok(())
    }
}

#[test]
fn test_get_manufacturer_too_many_properties() {
    let response = GetCapabilityResp {
        more_data: TpmiYesNo::NO,
        capability_data: TpmsCapabilityData::TpmProperties(
            TpmlTaggedTpmProperty::new(
                &[TpmsTaggedProperty {
                    property: TPM2PT::Manufacturer,
                    value: 4,
                }; 6],
            )
            .unwrap(),
        ),
    };

    let mut connection = FakeTpmConnection::default();
    connection.len = response.try_marshal(&mut connection.response).unwrap();
    let mut tpm = TpmClient::new(connection);

    assert_eq!(get_manufacturer_id(&mut tpm), Err(TPM_RC_SIZE.into()));
}

#[test]
fn test_get_manufacturer_wrong_type_properties() {
    let response = GetCapabilityResp {
        more_data: TpmiYesNo::NO,
        capability_data: TpmsCapabilityData::Algorithms(
            TpmlAlgProperty::new(&[TpmsAlgProperty {
                alg: TPM2AlgID::SHA256,
                alg_properties: TpmaAlgorithm::empty(),
            }])
            .unwrap(),
        ),
    };
    let mut connection = FakeTpmConnection::default();
    connection.len = response.try_marshal(&mut connection.response).unwrap();
    let mut tpm = TpmClient::new(connection);

    assert_eq!(
        get_manufacturer_id(&mut tpm),
        Err(TPM_RC_SELECTOR.into())
    );
}
