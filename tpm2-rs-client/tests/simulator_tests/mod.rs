mod common;
use common::*;

use tpm2_rs_client::TpmClient;
use tpm2_rs_base::{commands::StartupCmd, TPM2SU};

const TPM_SIMULATOR_ENV_VAR: &str = "TPM_RS_SIMULATOR";
fn get_simulator_path() -> String {
    std::env::var(TPM_SIMULATOR_ENV_VAR)
        .expect("Set TPM_RS_SIMULATOR to run tests against a simulator")
}

#[test]
fn test_startup_tpm() {
    let sim = TpmSimulator::new(&get_simulator_path()).unwrap();
    let startup = StartupCmd {
        startup_type: TPM2SU::Clear,
    };
    let mut client = TpmClient::new(sim.connect().unwrap());
    assert!(client.run_command(&startup).is_ok());
}

/*
TODO: Move to be a test under the feature client crate (remake feature client).
// If test_startup_tpm passes, this will not panic.
fn get_started_tpm() -> (TpmSimulator, TpmClient<TpmTcpConnection>) {
    let sim = TpmSimulator::new(&get_simulator_path()).unwrap();
    let startup = StartupCmd {
        startup_type: TPM2SU::Clear,
    };
    let mut client = TpmClient::new(sim.connect().unwrap());
    client.run_command(&startup, &mut tpm).unwrap();
    (sim, client)
}

#[test]
fn test_get_manufacturer_id() {
    let (sim, client) = get_started_tpm();
    assert!(get_manufacturer_id(&mut client).is_ok());
}
*/
