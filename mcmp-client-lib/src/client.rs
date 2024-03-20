use std::sync::mpsc;
use anyhow::Result;
use futures::lock::Mutex;
use mcmp_common::message::ClientMessage;

lazy_static::lazy_static!{
    pub static ref COMMS: Mutex<(mpsc::Sender<ClientMessage>, mpsc::Receiver<ClientMessage>)> = Mutex::new(mpsc::channel());
}

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