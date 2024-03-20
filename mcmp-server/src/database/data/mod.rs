use std::collections::HashMap;

use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DbData {
    pub users: HashMap<String, DbUser>
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DbUser {
    pub username: String,
    pub pw_hash: String,
}


impl DbData {
    pub fn new_user(&mut self, username: String, pw_hash: String) {
        self.users.insert(username.clone(), DbUser {
            username,
            pw_hash,
        });
    }
}

impl DbUser {
    pub fn set_passwd(&mut self, pw: String) -> anyhow::Result<()>{
        let h = pwhash::bcrypt::hash(pw)?;
        self.pw_hash = h;
        Ok(())
    }

    pub fn check_pw_hash(&self, hash: &String) -> bool {
        &self.pw_hash == hash
    }
}
