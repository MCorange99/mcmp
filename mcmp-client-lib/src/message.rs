use std::sync::mpsc;
use lazy_static::lazy_static;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};

lazy_static!{
    pub static ref COMMS: Mutex<(mpsc::Sender<ClientMessage>, mpsc::Receiver<ClientMessage>)> = Mutex::new(mpsc::channel());
}


#[derive(Debug, Clone)]
pub enum ClientMessage {

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsClientMessageType {
    Ping {
        time: u64
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsClientMessage {
    typ: WsClientMessageType,
    authorization: Option<String>,
}