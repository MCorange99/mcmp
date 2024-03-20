use serde::{Serialize, Deserialize};


#[derive(Debug, Clone)]
pub enum ClientMessage {

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsMessageType {
    CPing {
        time: u64
    },
    SPing {
        time: u64
    },
    CAuthReq {
        user: String,
        pw_hash: String
    },
    SAuthRes {
        token: Option<bool>,
        valid_until: usize // unix time
    },
    Error {
        msg: String,
        id: usize
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMessage {
    pub typ: WsMessageType,
    pub authorization: Option<String>,
}