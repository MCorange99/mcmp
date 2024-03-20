mod data;

use std::{fs::File, io::{Read, Seek, Write}, path::PathBuf};
use anyhow::Result;


#[derive(Debug)]
pub struct Database {
    fd: Option<File>,
    path: String,
    pub data: data::DbData
}


impl Database {
    pub fn new(p: String) -> Result<Self> {
        log::info!("Connecting to db");
        let mut s = Self {
            fd: None,
            path: p,
            data: data::DbData::default(),
        };
        s.open()?;
        Ok(s)
    }

    pub fn open(&mut self) -> Result<()> {
        let exists = PathBuf::from(&self.path).exists();
        self.fd = Some(
            std::fs::File::options()
                .read(true)
                .write(true)
                .create(true)
                .open(&self.path)?
        );
        
        if exists {
            self.refresh()?;
        } else {
            log::debug!("Database doesnt exist, creating it");
            self.flush()?;
        }

        Ok(())
    }

    pub fn flush(&self) -> Result<()> {
        let data = serde_json::to_string_pretty(&self.data)?;
        self.fd().set_len(0)?;
        self.fd().seek(std::io::SeekFrom::End(0))?;

        self.fd().write(data.as_bytes())?;
        self.fd().flush()?;
        Ok(())
    }

    pub fn refresh(&mut self) -> Result<()> {
        let mut data = String::new();
        self.fd().read_to_string(&mut data)?;
        self.data = serde_json::from_str(data.as_str())?;
        Ok(())
    }

    fn fd(&self) -> &File {
        self.fd.as_ref().unwrap()
    }
}