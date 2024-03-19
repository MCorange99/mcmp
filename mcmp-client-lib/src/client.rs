use std::{fmt::Debug, sync::mpsc};
use anyhow::Result;
pub struct MCMPClient {
    host: Option<String>
}

impl MCMPClient {
    pub fn new() -> Self {
        Self {
            host: None,
        }
    }
    
    pub fn host(&self) -> Option<&String> {
        self.host.as_ref()
    }
    
    pub fn set_host(&mut self, host: Option<String>) {
        self.host = host;
    }

    pub fn connect(&mut self) -> Result<()> {
        ws::listen(self.host.as_deref().expect("Set your host"), |a| {
            move |msg| {

                

                Ok(())   
            }
        })?;

        Ok(())
    }
}