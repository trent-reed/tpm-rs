use std::io::{Error, ErrorKind, IoSlice, Read, Result, Write};
use std::net::TcpStream;
use std::process::{Child, Command};
use tpm2_rs_client::TpmConnection;
use tpm2_rs_errors::TssErrorCode;
use tpm2_rs_errors::{TssResult, TssTcsError};
use zerocopy::big_endian::U32;
use zerocopy::AsBytes;

const SIMULATOR_IP: &str = "127.0.0.1";
// TODO: Either pass ports or get simulator to export ports for multithreaded-use.
const SIMULATOR_TPM_PORT: u16 = 2321;
const SIMULATOR_PLAT_PORT: u16 = 2322;

pub struct TpmSimulator {
    child_process: Child,
}

impl TpmSimulator {
    // Starts up the TPM simulator with a platform server listening at the give IP/port.
    fn start_tcp_tpm(ip: &str, plat_port: u16) -> Result<()> {
        let mut connection = TcpStream::connect(format!("{ip}:{plat_port}"))?;
        TpmCommand::SignalNvOff.issue_to_platform(&mut connection)?;
        TpmCommand::SignalPowerOff.issue_to_platform(&mut connection)?;
        TpmCommand::SignalPowerOn.issue_to_platform(&mut connection)?;
        TpmCommand::SignalNvOn.issue_to_platform(&mut connection)
    }
    pub fn new(simulator_bin: &str) -> Result<Self> {
        let child_process = Command::new(format!(".{simulator_bin}"))
            .current_dir("/")
            .spawn()?;
        let mut attempts = 0;
        while let Err(err) = Self::start_tcp_tpm(SIMULATOR_IP, SIMULATOR_PLAT_PORT) {
            if attempts == 3 {
                return Err(err);
            }
            attempts += 1;
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        Ok(Self { child_process })
    }
    pub fn connect(&self) -> Result<TpmTcpConnection> {
        TpmTcpConnection::new(SIMULATOR_IP, SIMULATOR_TPM_PORT)
    }
}
impl Drop for TpmSimulator {
    fn drop(&mut self) {
        if let Err(x) = self.child_process.kill() {
            println!("Failed to stop simulator: {x}");
        }
    }
}

#[repr(u32)]
enum TpmCommand {
    SignalPowerOn = 1,
    SignalPowerOff = 2,
    SendCommand = 8,
    SignalNvOn = 11,
    SignalNvOff = 12,
}
impl TpmCommand {
    // Issues a platform TPM command on the given TCP stream.
    pub fn issue_to_platform(self, connection: &mut TcpStream) -> Result<()> {
        connection.write_all(U32::from(self as u32).as_bytes())?;
        let mut rc = U32::ZERO;
        connection.read_exact(rc.as_bytes_mut())?;
        if rc != U32::ZERO {
            Err(Error::new(
                ErrorKind::Other,
                format!("Platform command error {}", rc.get()),
            ))
        } else {
            Ok(())
        }
    }
}

#[derive(AsBytes)]
#[repr(C, packed)]
struct TcpTpmHeader {
    tcp_cmd: U32,
    locality: u8,
    cmd_len: U32,
}
// Provides TCP transport for talking to a TPM simulator.
pub struct TpmTcpConnection {
    tpm_conn: TcpStream,
}
impl TpmTcpConnection {
    pub fn new(ip: &str, tpm_port: u16) -> Result<Self> {
        Ok(Self {
            tpm_conn: TcpStream::connect(format!("{ip}:{tpm_port}"))?,
        })
    }

    fn read_tpm_u32(&mut self) -> TssResult<u32> {
        let mut val = U32::ZERO;
        self.tpm_conn
            .read_exact(val.as_bytes_mut())
            .map_err(|_| TssTcsError::new(TssErrorCode::OutOfMemory))?;
        Ok(val.get())
    }
}
impl TpmConnection for TpmTcpConnection {
    fn transact(&mut self, command: &[u8], response: &mut [u8]) -> TssResult<()> {
        let cmd_size: u32 = command
            .len()
            .try_into()
            .map_err(|_| TssTcsError::new(TssErrorCode::OutOfMemory))?;
        let tcp_hdr = TcpTpmHeader {
            tcp_cmd: U32::new(TpmCommand::SendCommand as u32),
            locality: 0,
            cmd_len: U32::new(cmd_size),
        };
        let txed = self
            .tpm_conn
            .write_vectored(&[IoSlice::new(tcp_hdr.as_bytes()), IoSlice::new(command)]);
        if txed.unwrap_or(0) != tcp_hdr.as_bytes().len() + command.len() {
            return Err(TssTcsError::new(TssErrorCode::OutOfMemory).into());
        }

        // Response contains a u32 size, the TPM response, and then an always-zero u32 trailer.
        let resp_size = self.read_tpm_u32()?;
        if resp_size as usize > response.len() {
            return Err(TssTcsError::new(TssErrorCode::OutOfMemory).into());
        }
        self.tpm_conn
            .read_exact(&mut response[..resp_size as usize])
            .map_err(|_| TssTcsError::new(TssErrorCode::OutOfMemory))?;
        if self.read_tpm_u32()? != 0 {
            return Err(TssTcsError::new(TssErrorCode::OutOfMemory).into());
        }
        Ok(())
    }
}
