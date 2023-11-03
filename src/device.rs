use std::io::{self, Write, Read};
use crate::command::TRIGgerCommand;
use crate::command::WAVeformCommand::RecieveData;
use crate::command::WAVeformCommand::MaxTransferSize;

pub trait Visa{
    fn write_scip_cmd(&mut self, buf: &[u8]) -> std::io::Result<()>;
    fn read_result(&mut self, timeout_s: u64) -> std::io::Result<String>;
    fn read_result2(&mut self) -> std::io::Result<String>;
    fn read_bytes_u8(&mut self, timeout_s: u64, data: &mut RecieveData) -> std::result::Result<(), Box<dyn std::error::Error>>;
    fn read_bytes_u16(&mut self, timeout_s: u64, data: &mut RecieveData) -> std::result::Result<(), Box<dyn std::error::Error>>;
}

impl Visa for std::net::TcpStream {
    fn write_scip_cmd(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.write_all(buf)?;
        self.flush()?;
        return Ok(())
    }
    fn read_result(&mut self, timeout_s: u64) -> std::io::Result<String> {
        self.set_read_timeout(Some(std::time::Duration::from_secs(timeout_s)))?;
        let mut buffer = String::new();
        let _ = self.read_to_string(&mut buffer);
        if buffer.ends_with('\n') {
            buffer.pop();
        }
        Ok(buffer)
    }

    fn read_result2(&mut self) -> std::io::Result<String> {
        let mut buffer = [0; 5];
        let n  = self.read(&mut buffer)?;
        let response = String::from_utf8_lossy(&buffer[..n]);
        Ok(response.to_string())
    }

    fn read_bytes_u8(&mut self, timeout_s: u64, data: &mut RecieveData) -> std::result::Result<(),Box<dyn std::error::Error>>{
        let mut magic = [0; 1];
        let mut header_len = [0; 1];
        self.read(&mut magic)?;
        self.read(&mut header_len)?;

        if magic[0] != b'#' {
            return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "Invalid header")));
        }
        let header_length: usize = (header_len[0] as char).to_digit(10)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid header length"))? as usize;

        let mut header = vec!(0; header_length);
        self.read(&mut header)?;
        let data_length_str: &str = std::str::from_utf8(&header)?;
        let data_length: usize = data_length_str.parse()?;
        match data {
            RecieveData::BYTE(ref mut vec) => {
                if (data_length) > MaxTransferSize::BYTE as usize{
                    return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "Incomplete data")));
                }
                //println!("data_length = {}, vec_len = {}", data_length, vec.len());
                let mut total_read: usize = 0;
                while total_read < data_length{
                    let size = match self.read(&mut vec[total_read..]){
                        Ok(s) => s,
                        Err(err) => {
                            println!("err {}", err);
                            break;
                        }
                    };
                    
                    //println!("read_size = {:?}", size);
                    total_read = total_read + size;
                    if size == 0 {
                        println!("0 dayo");
                        break;
                    }
                    if total_read == MaxTransferSize::BYTE as usize {
                        let mut dummy =  [0; 1];
                        self.read(&mut dummy)?;
                    }
                }
                //let size = self.read(vec)?;
                //if size == 0 {
                //    return Err(Box::new(io::Error::new(io::ErrorKind::UnexpectedEof, "No data received")));
                //}
                 
                println!("size = {:?}", total_read);
                if total_read != MaxTransferSize::BYTE as usize {
                    if vec[total_read - 1] == b'\n' {
                        vec[total_read -1] = 0;
                    }
                    else {
                        return Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, "the suffix of the wavedata has to be '\n'")));
                    }
                }
            }
            _ => return Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, "Invalid data type"))),
        }
        Ok(())
    }


    fn read_bytes_u16(&mut self, timeout_s: u64, data: &mut RecieveData) -> std::result::Result<(), Box<dyn std::error::Error>>{
        let mut buffer = Vec::new();
        self.set_read_timeout(Some(std::time::Duration::from_secs(timeout_s)))?;
        self.read_to_end(&mut buffer)?;
    
        if buffer.len() < 2 || buffer[0] != b'#' {
            return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "Invalid header")));
        }
        let header_length: usize = (buffer[1] as char).to_digit(10)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid header length"))? as usize;
        if buffer.len() < 2 + header_length {
            return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "Incomplete header")));
        }
    
        let data_length_str: &str = std::str::from_utf8(&buffer[2..2 + header_length])?;
        let data_length: usize = data_length_str.parse()?;
        if buffer.len() < 2 + header_length + data_length {
            return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "Incomplete data")));
        }
        let start = 2 + header_length;
        let end = start + data_length * 2; 
        let data_bytes = &buffer[start..end];
        if data_bytes.len() % 2 != 0 {
            return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "Data length is not even")));
        }

        if data_length > MaxTransferSize::WORD as usize {
            return Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, "invalid length")));
        }

        match data {
            RecieveData::WORD(ref mut vec) => {
                for (i, chunk) in data_bytes.chunks_exact(2).enumerate() {
                    let num = u16::from_le_bytes([chunk[0], chunk[1]]);
                    vec[i] = num;
                }
            }
            _ => return Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, "Invalid data type"))),
        }

        //let mut u16_data = Vec::new();
        //for chunk in data.chunks_exact(2) {
        //    let num = u16::from_le_bytes([chunk[0], chunk[1]]);
        //    u16_data.push(num);
        //}
        Ok(())
    }

}
