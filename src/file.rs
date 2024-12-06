use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
};

const SQLITE_HEADER_SIZE: usize = 100;
const SQLITE_SCHEMA_TABLE: &str = "sqlite_schema";

pub struct Bambang {
    pub file: File,
    pub page_size: u16,
}

impl Bambang {
    pub fn new(path: &str) -> io::Result<Bambang> {
        let mut file = File::open(path)?;
        let mut header = [0u8; SQLITE_HEADER_SIZE];

        file.read_exact(&mut header)?;

        if &header[0..16] != b"SQLite format 3\0" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid SQLite header",
            ));
        }

        let page_size = u16::from_be_bytes(header[16..18].try_into().unwrap());
        Ok(Bambang { file, page_size })
    }

    pub fn read_page(&mut self, page_number: u32) -> io::Result<Vec<u8>> {
        let offset = (page_number as u64 - 1) * self.page_size as u64;
        self.file.seek(SeekFrom::Start(offset))?;
        let mut buffer = vec![0u8; self.page_size as usize];
        self.file.read_exact(&mut buffer)?;
        Ok(buffer)
    }
}
