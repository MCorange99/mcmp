pub mod error;

use std::collections::HashMap;
use mcmp_common::{error::ErrorId, message::{WsMessage, WsMessageType}};

use self::error::HandlerError;

use super::{ServerData, WsServerDataContainer};


#[derive(Debug, Clone, Default)]
pub struct MessageHandler {
    authed_users: HashMap<usize, String>
}


impl MessageHandler {
    pub fn new() -> Self {
        Self {
            authed_users: HashMap::new(),
        }
    }
    pub async fn handle(&mut self, msg: WsMessage, uid: usize, data: &WsServerDataContainer<ServerData>) -> Result<(), HandlerError> {
        match msg.typ {
            WsMessageType::CAuthReq { user,pw_hash} => {
                self.handle_authreq(uid, data, user, pw_hash).await?;
            },
            _ => () 
        }
        
        self.check_auth(uid, msg.authorization).await?;

        Ok(())
    }

    async fn handle_authreq(&mut self, uid: usize, data: &WsServerDataContainer<ServerData>, user: String, pw_hash: String) -> Result<(), HandlerError> {
        let db = {&data.read().await.database};

        let Some(db_user) = db.data.users.get(&user) else {
            return Err(HandlerError::new("User not found", ErrorId::UserNotFound));
        };

        if db_user.check_pw_hash(&pw_hash){
            self.authed_users.insert(uid, pw_hash);
        } else {
            return Err(HandlerError::new("Wrong password", ErrorId::WrongPasswd));
        }

        Ok(())
    }


    async fn check_auth(&self, uid: usize, auth: Option<String>) -> Result<&String, HandlerError> {

        let Some(auth) = auth else {
            return Err(HandlerError::new("Not authorized", ErrorId::NotAuthed));
        };

        let Some(auth2) = self.authed_users.get(&uid) else {
            return Err(HandlerError::new("Not authorized", ErrorId::NotAuthed));
        };

        if &auth != auth2 {
            return Err(HandlerError::new("Not authorized", ErrorId::NotAuthed));
        }

        Ok(auth2)
    }
}