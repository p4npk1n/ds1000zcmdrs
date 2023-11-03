use std::str::FromStr;
use std::fmt;
use crate::device::Visa;
use std::io;
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IoError(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SWEep{
    AUTO,
    NORM,
    SING,
}

impl fmt::Display for SWEep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            SWEep::AUTO => "AUTO",
            SWEep::NORM => "NORM",
            SWEep::SING => "SING",
        })
    }
}

impl FromStr for SWEep {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AUTO" => Ok(SWEep::AUTO),
            "NORM" => Ok(SWEep::NORM),
            "SING" => Ok(SWEep::SING),
            _ => Err(Error::IoError(io::Error::new(io::ErrorKind::InvalidData, "Invalid SWEep"))),
        }
    }
}

#[derive(Debug)]
pub struct TRIGgerCommand{
    pub device: std::net::TcpStream,
    pub sweep: SWEep,
}

impl TRIGgerCommand {
    pub fn get_sweep(&mut self) -> Result<SWEep>{
        self.device.write_scip_cmd(b":TRIGger:SWEep?\n")?;
        let buffer :String = self.device.read_result(1)?;
        let swp = buffer.trim_end_matches('\n');
        println!("swp = {}", swp);
        return swp.parse()
    }

    pub fn new(device: std::net::TcpStream) -> Result<TRIGgerCommand> {
        let mut cmd = TRIGgerCommand {
            device: device,
            sweep: SWEep::AUTO,
        };
        cmd.sweep = cmd.get_sweep()?;
        Ok(cmd)
    }
}
