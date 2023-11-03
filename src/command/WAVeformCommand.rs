use std::fmt;
use std::io::{self};
use crate::device::Visa;
use crate::command::TRIGgerCommand;
use std::str::FromStr;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    CanNotChangeMode(Mode),
    ParseFloatError(std::num::ParseFloatError),
    ParseIntError(std::num::ParseIntError),
    ExceededMaxMemorySize(MaxMemorySize),
    StartIsGreaterThanStop(u32, u32),
    ExceedeMaxTransferSize(MaxTransferSize),
    CustomError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError(err) => write!(f, "IO error: {}", err),
            Error::CanNotChangeMode(err) => write!(f, "The mode MAX and RAW has to set the triger mode to SINGLE: {}", err),
            Error::ParseFloatError(err) => write!(f, "Can not convert from the string is from result of scip cmmand {} to f32", err),
            Error::ParseIntError(err) => write!(f, "Can not convert from the string is from result of scip cmmand {} to u32", err),
            Error::ExceededMaxMemorySize(memory_size) => write!(f, "Exceeded the max memory size {:?}", memory_size),
            Error::StartIsGreaterThanStop(start, stop) => write!(f, "the start point {} is greater than the stop point {}", start, stop),
            Error::ExceedeMaxTransferSize(transfer_size) => write!(f, "Exceeded the max transfer size {:?}", transfer_size),
            Error::CustomError(e) => write!(f, "Custom error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IoError(err) => Some(err),
            Error::CanNotChangeMode(_) => None,
            Error::ParseFloatError(err) => Some(err),
            Error::ParseIntError(err) => Some(err),
            Error::ExceededMaxMemorySize(_) => None, 
            Error::StartIsGreaterThanStop(_, _) => None,
            Error::ExceedeMaxTransferSize(_) => None,
            Error::CustomError(_) => None,
        }
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(err: std::num::ParseFloatError) -> Self {
        Error::ParseFloatError(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::ParseIntError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<Mode> for Error {
    fn from(err: Mode) -> Self {
        Error::CanNotChangeMode(err)
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        Error::CustomError(format!("Internal error: {}", err))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryDepth {
    DS1102Z_E = 24000000, // 24Mpts
}

impl fmt::Display for MemoryDepth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            MemoryDepth::DS1102Z_E => "24Mpts",
        })
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Source {
    D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15,
    CHAN1, CHAN2, CHAN3, CHAN4, MATH,
}

impl FromStr for Source {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "D0" => Ok(Source::D0),
            "D1" => Ok(Source::D1),
            "D2" => Ok(Source::D2),
            "D3" => Ok(Source::D3),
            "D4" => Ok(Source::D4),
            "D5" => Ok(Source::D5),
            "D6" => Ok(Source::D6),
            "D7" => Ok(Source::D7),
            "D8" => Ok(Source::D8),
            "D9" => Ok(Source::D9),
            "D10" => Ok(Source::D10),
            "D11" => Ok(Source::D11),
            "D12" => Ok(Source::D12),
            "D13" => Ok(Source::D13),
            "D14" => Ok(Source::D14),
            "D15" => Ok(Source::D15),
            "CHAN1" => Ok(Source::CHAN1),
            "CHAN2" => Ok(Source::CHAN2),
            "CHAN3" => Ok(Source::CHAN3),
            "CHAN4" => Ok(Source::CHAN4),
            "MATH" => Ok(Source::MATH),
            _ => Err(Error::IoError(io::Error::new(io::ErrorKind::InvalidData, "Invalid source"))),
        }
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Source::D0 => "D0",
            Source::D1 => "D1",
            Source::D2 => "D2",
            Source::D3 => "D3",
            Source::D4 => "D4",
            Source::D5 => "D5",
            Source::D6 => "D6",
            Source::D7 => "D7",
            Source::D8 => "D8",
            Source::D9 => "D9",
            Source::D10 => "D10",
            Source::D11 => "D11",
            Source::D12 => "D12",
            Source::D13 => "D13",
            Source::D14 => "D14",
            Source::D15 => "D15",
            Source::CHAN1 => "CHAN1",
            Source::CHAN2 => "CHAN2",
            Source::CHAN3 => "CHAN3",
            Source::CHAN4 => "CHAN4",
            Source::MATH => "MATH",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    NORM, MAX, RAW,
}

impl FromStr for Mode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "NORM" => Ok(Mode::NORM),
            "MAX" => Ok(Mode::MAX),
            "RAW" => Ok(Mode::RAW),
            _ => Err(Error::IoError(io::Error::new(io::ErrorKind::InvalidData, "Invalid mode"))),
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Mode::NORM => "NORM",
            Mode::MAX => "MAX",
            Mode::RAW => "RAW",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaxMemorySize {
    NORM(i32),
    MAX(MemoryDepth),
    RAW(MemoryDepth),
}

impl fmt::Display for MaxMemorySize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MaxMemorySize::NORM(val) => write!(f, "NORM({})", val),
            MaxMemorySize::MAX(depth) => write!(f, "MAX({})", depth),
            MaxMemorySize::RAW(depth) => write!(f, "RAW({})", depth),
        }
    }
}


impl MaxMemorySize {
    fn new(mode: Mode, memory_depth: MemoryDepth) -> MaxMemorySize {
        match mode {
            Mode::MAX => return MaxMemorySize::MAX(memory_depth),
            Mode::RAW => return MaxMemorySize::MAX(memory_depth),
            Mode::NORM => return MaxMemorySize::NORM(1200),
        }
    }

    fn to_u32(&self) -> u32 {
        match *self {
            MaxMemorySize::NORM(val) => val as u32,
            MaxMemorySize::MAX(depth) => depth as u32, 
            MaxMemorySize::RAW(depth) => depth as u32, 
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Format {
    WORD, BYTE, ASC,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Format::WORD => "WORD",
            Format::BYTE => "BYTE",
            Format::ASC => "ASC",
        })
    }
}

impl FromStr for Format {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "WORD" => Ok(Format::WORD),
            "BYTE" => Ok(Format::BYTE),
            "ASC" => Ok(Format::ASC),
            _ => Err(Error::IoError(io::Error::new(io::ErrorKind::InvalidData, "Invalid mode"))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaxTransferSize {
    WORD = 125000,
    BYTE = 250000,
    ASC = 15625,
}


impl fmt::Display for MaxTransferSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            MaxTransferSize::WORD => "WORD (125000)",
            MaxTransferSize::BYTE => "BYTE (250000)",
            MaxTransferSize::ASC => "ASC (15625)",
        })
    }
}


impl From<Format> for MaxTransferSize {
    fn from(format: Format) -> Self {
        match format {
            Format::WORD => MaxTransferSize::WORD,
            Format::BYTE => MaxTransferSize::BYTE,
            Format::ASC => MaxTransferSize::ASC,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TwoDiv<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RecieveData {
    ASC(Vec<String>),
    WORD(Vec<u16>),
    BYTE(Vec<u8>),
}

impl RecieveData {
    pub fn new(data_type: MaxTransferSize) -> Self {
        match data_type {
            MaxTransferSize::WORD => RecieveData::WORD(vec![0; MaxTransferSize::WORD as usize]),
            MaxTransferSize::BYTE => RecieveData::BYTE(vec![0; MaxTransferSize::BYTE as usize]),
            MaxTransferSize::ASC => RecieveData::ASC(vec![String::new(); MaxTransferSize::ASC as usize]),
        }
    }
}

//#[derive(Debug, Clone, PartialEq)]
//pub enum ConvertData {
//    ASC(Vec<TwoDiv<f32>>),
//    WORD(Vec<TwoDiv<f32>>),
//    BYTE(Vec<TwoDiv<f32>>),
//}

pub struct ConvertData {
    pub data: Vec<TwoDiv<f32>>,
    pub count: u32,
}

impl ConvertData {
    pub fn new() -> Self {
        return ConvertData{ data : vec![TwoDiv{x: 0.0, y: 0.0}; MemoryDepth::DS1102Z_E as usize], count: 0};
    }

    pub fn convert_voltage(&mut self, wavedata: &WAVeformCommands) -> Result<()> {
        match &wavedata.data{
            RecieveData::ASC(recv) => {
                Ok(())                
            }
            RecieveData::BYTE(recv) => {
                let mut point: u32 = 0;
                let start = wavedata.start_point as u32;
                let stop = wavedata.stop_point as u32;
                let size: u32 = stop - start + 1;
                println!("convert size = {}, {}, {}, {}", self.count, size, wavedata.origin.y, wavedata.reference.y);
                let data_slice = &mut self.data.get_mut(self.count as usize..(self.count + size) as usize);
                match data_slice {
                    Some(data_some) => {
                        for (re, cd) in recv.iter().zip(data_some.iter_mut()) {
                            cd.y = (*re as f32 - wavedata.origin.y - wavedata.reference.y) * wavedata.increment.y;
                            cd.x = wavedata.origin.x + point as f32 * wavedata.increment.x; 
                            point = point + 1;
                            if point > size{
                                break;
                            }
                        }
                        println!("sizeaaa = {}, count = {}", size, self.count);
                        self.count = self.count + size as u32;
                        println!("sizeaaa = {}, count = {}", size, self.count);
                        Ok(())
                    }
                    None => {
                        println!("sizeaaa = none");
                            Ok(())
                    }
                }
            }
            RecieveData::WORD(recv) => {
                Ok(())
            }
            _ => {
                Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub struct WAVeformCommands {
    pub device: std::net::TcpStream,
    pub memory_depth: MemoryDepth,
    pub max_transfer_size: MaxTransferSize,
    pub data: RecieveData,
    pub start_point: u32,
    pub stop_point: u32,
    pub origin: TwoDiv<f32>,
    pub reference: TwoDiv<f32>,
    pub increment: TwoDiv<f32>,
    pub source: Source,
    pub format: Format,
    pub max_memory_size: MaxMemorySize,
    pub mode: Mode,
}

impl WAVeformCommands {
    pub fn new(device: std::net::TcpStream, memory_depth: MemoryDepth, trigger: & TRIGgerCommand::TRIGgerCommand) -> Result<WAVeformCommands> {
        let mut cmd = WAVeformCommands {
            device: device,
            memory_depth: memory_depth,
            max_transfer_size: MaxTransferSize::WORD,
            data: RecieveData::ASC(Vec::new()) ,
            start_point: 0,
            stop_point: 0,
            origin: TwoDiv { x: 0.0, y: 0.0 },
            reference: TwoDiv { x: 0.0, y: 0.0 },
            increment: TwoDiv { x: 0.0, y: 0.0 },
            source: Source::CHAN1,
            format: Format::ASC,
            max_memory_size: MaxMemorySize::new(Mode::MAX, memory_depth),
            mode: Mode::MAX
        };
        cmd.get_origin()?;
        cmd.get_reference()?;
        cmd.get_increment()?;
        cmd.get_mode(trigger)?;
        cmd.get_source()?;
        cmd.get_format()?;
        cmd.start(1)?;
        cmd.stop(1)?;
        Ok(cmd)

    }

    pub fn set_source(&mut self, source: Source) ->  Result<()>{
        let command = format!(":WAVeform:SOURce {}\n", source);
        self.device.write_scip_cmd(command.as_bytes())?;
        Ok(())
    }

    pub fn get_source(&mut self) -> Result<()> {
        self.device.write_scip_cmd(b":WAVeform:SOURce?\n")?;
        let buffer :String = self.device.read_result(1)?;
        self.source = buffer.parse()?;
        Ok(())
    }

    pub fn set_mode(&mut self, mode: Mode) -> Result<()> {
        let command = format!(":WAVeform:MODE {}\n", mode);
        self.device.write_scip_cmd(command.as_bytes())?;
        Ok(())
    }       

    pub fn get_mode(&mut self, trigger :&TRIGgerCommand::TRIGgerCommand) -> Result<()> {
        self.device.write_scip_cmd(b":WAVeform:MODE?\n")?;
        let buffer :String = self.device.read_result(1)?;
        let mode: Mode = buffer.parse()?;
        if (mode == Mode::MAX || mode == Mode::RAW) && trigger.sweep != TRIGgerCommand::SWEep::SING{
            return Err(Error::CanNotChangeMode(mode));
        }
        self.max_memory_size = MaxMemorySize::new(mode, self.memory_depth);
        self.mode = mode;
        return Ok(());
    }

    pub fn mode(&mut self, mode: Mode, trigger :&TRIGgerCommand::TRIGgerCommand) -> Result<()> {
        self.set_mode(mode)?;
        self.get_mode(trigger)
    }

    pub fn set_format(&mut self, format: Format) -> Result<()> {
        let command = format!(":WAVeform:FORMat {}\n", format);
        self.device.write_scip_cmd(command.as_bytes())?;
        Ok(())
    }   

    pub fn get_format(&mut self) -> Result<()> {
        self.device.write_scip_cmd(b":WAVeform:FORMat?\n")?;
        let buffer: String = self.device.read_result(1)?;
        let format: Format = buffer.parse()?;
        self.max_transfer_size = MaxTransferSize::from(format);
        self.data = RecieveData::new(self.max_transfer_size);
        self.format = format;
        Ok(())
    }

    pub fn format(&mut self, format: Format) -> Result<()> {
        println!("format start");
        self.set_format(format)?;
        println!("format mid");
        self.get_format()?;
        println!("format end");
        Ok(())
    }

    pub fn get_xorigin(&mut self) -> Result<()> {
        self.device.write_scip_cmd(b":WAVeform:XORigin?\n")?;
        let buffer: String = self.device.read_result(1)?;
        self.origin.x = buffer.parse::<f32>()?;
        Ok(())
    }

    pub fn get_yorigin(&mut self) -> Result<()> {
        self.device.write_scip_cmd(b":WAVeform:YORigin?\n")?;
        let buffer: String = self.device.read_result(1)?;
        self.origin.y = buffer.parse::<f32>()?;
        Ok(())
    }

    pub fn get_origin(&mut self) -> Result<()> {
        self.get_xorigin()?;
        self.get_yorigin()?;
        Ok(())
    }

    pub fn get_xreference(&mut self) -> Result<()> {
        self.device.write_scip_cmd(b":WAVeform:XREFerence?\n")?;
        let buffer: String = self.device.read_result(1)?;
        self.reference.x = buffer.parse::<f32>()?;
        Ok(())
    }

    pub fn get_yreference(&mut self) -> Result<()> {
        self.device.write_scip_cmd(b":WAVeform:YREFerence?\n")?;
        let buffer: String = self.device.read_result(1)?;
        self.reference.y = buffer.parse::<f32>()?;
        Ok(())
    }

    pub fn get_reference(&mut self) -> Result<()> {
        self.get_xreference()?;
        self.get_yreference()?;
        Ok(())
    }

    pub fn get_xincrement(&mut self) -> Result<()> {
        self.device.write_scip_cmd(b":WAVeform:XINCrement?\n")?;
        let buffer: String = self.device.read_result(1)?;
        self.increment.x = buffer.parse::<f32>()?;
        Ok(())
    }

    pub fn get_yincrement(&mut self) -> Result<()> {
        self.device.write_scip_cmd(b":WAVeform:YINCrement?\n")?;
        let buffer: String = self.device.read_result(1)?;
        self.increment.y = buffer.parse::<f32>()?;
        Ok(())
    }

    pub fn get_increment(&mut self) -> Result<()> {
        self.get_xincrement()?;
        self.get_yincrement()?;
        Ok(())
    }

    pub fn get_start_point(&mut self) -> Result<()> {
        self.device.write_scip_cmd(b":WAVeform:STARt?\n")?;
        let buffer: String = self.device.read_result(1)?;
        println!("startpoint = {}", buffer);
        self.start_point = buffer.parse::<u32>()?;
        Ok(())
    }

    pub fn set_start_point(&mut self, start_point: u32) -> Result<()> {
        if start_point > self.max_memory_size.to_u32() {
            return Err(Error::ExceededMaxMemorySize(self.max_memory_size));
        }
        let command = format!(":WAVeform:STARt {}\n", start_point);
        self.device.write_scip_cmd(command.as_bytes())?;
        Ok(())
    }

    pub fn start(&mut self, start_point: u32) -> Result<()> {
        self.set_start_point(start_point)?;
        self.get_start_point()?;
        Ok(())
    }

    pub fn get_stop_point(&mut self) -> Result<()> {
        self.device.write_scip_cmd(b":WAVeform:STOP?\n")?;
        let buffer: String = self.device.read_result(1)?;
        self.stop_point = buffer.parse::<u32>()?;
        Ok(())
    }

    pub fn set_stop_point(&mut self, stop_point: u32) -> Result<()> {
        if stop_point > self.max_memory_size.to_u32() {
            return Err(Error::ExceededMaxMemorySize(self.max_memory_size));
        }
        if (stop_point < self.start_point) {
            return Err(Error::StartIsGreaterThanStop(self.start_point, self.stop_point));
        }
        if (stop_point - self.start_point) > self.max_transfer_size as u32 {
            println!("erro stop_point = {}, start = {}, max = {}", stop_point, self.start_point, self.max_transfer_size);
            return Err(Error::ExceedeMaxTransferSize(self.max_transfer_size));
        }
        let command = format!(":WAVeform:STOP {}\n", stop_point);
        self.device.write_scip_cmd(command.as_bytes())?;
        Ok(())
    }

    pub fn stop(&mut self, stop_point: u32) -> Result<()> {
        self.set_stop_point(stop_point)?;
        self.get_stop_point()?;
        Ok(())
    }

    pub fn get_data(&mut self) -> Result<()> {
        self.device.write_scip_cmd(b":WAVeform:DATA?\n")?;
        match self.format {
            Format::BYTE => {
                println!("fadasfafdad");
                self.device.read_bytes_u8(10, &mut self.data)?;
            }
            Format::WORD => {
                self.device.read_bytes_u16(10, &mut self.data)?;
            }
            _ => {
                return Ok(());
            }
        }
        Ok(())
    }


    
}


pub fn get_data(range: u32,  waveform: &mut WAVeformCommands, convert_data: &mut ConvertData) -> Result<()>{
    let count = (range + waveform.max_transfer_size as u32 - 1) / waveform.max_transfer_size as u32;
    println!("count = {}", count);
    for i in 0..count as usize {
        let a1: u32 = 1;
        let n: u32 = i as u32;
        let d: u32 = waveform.max_transfer_size as u32;
        let start_n: u32 = n * d + a1;
        let end_n: u32 = ((start_n + d - 1).min(range)).min(start_n + d - 1);
        println!("start = {}, end = {}", start_n, end_n);
        waveform.start(start_n)?;
        waveform.stop(end_n)?;
        waveform.get_data()?;
        convert_data.convert_voltage(waveform)?;
    }


    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    #[test]
    fn test_set_mode() {
        let address = "169.254.245.109:5555";
        let memory_depth = MemoryDepth::DS1102Z_E;
        let device_trigger = match std::net::TcpStream::connect(address) {
            Ok(stream) => stream,
            Err(e) => {
                panic!("Failed to connect to device: {}", e);
            }
        };
        let device_wafeform: std::net::TcpStream = device_trigger.try_clone().unwrap();
        let trigger_command = TRIGgerCommand::TRIGgerCommand::new(device_trigger).unwrap();
        //println!("{:?}", trigger_command);
        let mut waveform_commands = WAVeformCommands::new(device_wafeform, memory_depth, &trigger_command).unwrap();
        {
            waveform_commands.format(Format::BYTE);
            waveform_commands.mode(Mode::RAW, &trigger_command);
            let mut convert_data = ConvertData::new();
            let range = 24000000;
            get_data(range, &mut waveform_commands, &mut convert_data).unwrap();
            let mut file = std::fs::File::create("output.bin").unwrap();
            println!("convert_data count = {}", convert_data.count);
            if let Some(slice) = convert_data.data.get(0..convert_data.count as usize) {
                for d in slice {
                    let s = format!("{}, {}\n", d.x, d.y);
                    file.write_all(s.as_bytes()).unwrap();
                }
            }
        }
    }
}