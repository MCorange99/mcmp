use std::{collections::HashMap, sync::{atomic::AtomicUsize, Arc}};

use tokio::sync::{mpsc, RwLock};
use warp::filters::ws::Message;

use crate::{config::Config, database::Database};


pub static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug, Clone)]
pub struct User {
    pub ws: mpsc::UnboundedSender<Message>,
    pub authed: bool
}


#[derive(Debug)]
pub struct ServerData {
    pub users: HashMap<usize, User>,
    pub config: Config,
    pub database: Database
}

impl ServerData {
    pub fn new(c: Config) -> anyhow::Result<Self> {
        Ok(Self {
            users: Default::default(),
            database: Database::new(c.database.uri.clone())?,
            config: c,
        })
    }
}

pub type WsServerDataContainer<T> = Arc<RwLock<T>>;
