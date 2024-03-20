mod message_handler;
mod data;

use std::sync::{atomic::Ordering, Arc};

use futures_util::{SinkExt, StreamExt, TryFutureExt};
use mcmp_common::message::{WsMessage, WsMessageType};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use warp::Filter;

use crate::{config::Config, server::data::*};

use self::message_handler::{error::HandlerError, MessageHandler};


pub async fn start_listening(config: Config) -> anyhow::Result<()> {
    
    let data: Arc<RwLock<ServerData>> = Arc::new(RwLock::new(ServerData::new(config.clone())?));
    // Turn our "state" into a new Filter...
    let data = warp::any().map(move || data.clone());

    let handler: Arc<RwLock<MessageHandler>> = Arc::new(RwLock::new(MessageHandler::new()));
    // Turn our "state" into a new Filter...
    let handler = warp::any().map(move || handler.clone());

    let chat = warp::path("ws")
    // The `ws()` filter will prepare Websocket handshake...
    .and(warp::ws())
    .and(data)
    .and(handler)
    .map(|ws: warp::ws::Ws, users, handler| {
        // This will call our function if the handshake succeeds.
        ws.on_upgrade(move |socket| on_connect(socket, users, handler))
    });


    // GET / -> index html
    let index = warp::path::end().map(|| warp::reply::reply());
    let routes = index.or(chat);
    log::info!("Listenign on 0.0.0.0:{}", config.port);
    warp::serve(routes).run(([127, 0, 0, 1], config.port)).await;
    Ok(())
}



async fn on_connect(ws: WebSocket, data: WsServerDataContainer<ServerData>, handler: WsServerDataContainer<MessageHandler>) {
    // Use a counter to assign a new unique ID for this user.
    let uid = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);

    // eprintln!("new chat user: {}", my_id);
    log::info!("User connected, id: {}", uid);

    // Split the socket into a sender and receive of messages.
    let (mut user_ws_tx, mut user_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the websocket...
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            user_ws_tx
                .send(message)
                .unwrap_or_else(|e| {
                    eprintln!("websocket send error: {}", e);
                })
                .await;
        }
    });

    // Save the sender in our list of connected users.
    data.write().await.users.insert(uid, User {
        ws: tx,
        authed: false,
    });

    // Return a `Future` that is basically a state machine managing
    // this specific user's connection.

    // Every time the user sends a message, broadcast it to
    // all other users...
    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                log::error!("websocket error(uid={}): {}", uid, e);
                break;
            }
        };
        on_message(uid, msg, &data, &handler).await;
    }

    // user_ws_rx stream will keep processing as long as the user stays
    // connected. Once they disconnect, then...
    on_disconnect(uid, &data).await;
}

async fn on_disconnect(uid: usize, data: &WsServerDataContainer<ServerData>) {
    log::warn!("User #{uid} disconnected");
    data.write().await.users.remove(&uid);
}

async fn on_message(uid: usize, msg: Message, data: &WsServerDataContainer<ServerData>, handler: &WsServerDataContainer<MessageHandler>) {
    // Skip any non-Text messages...
    let Ok(msg) = bincode::deserialize::<WsMessage>(msg.as_bytes()) else {
        log::warn!("Failed to parse message from User ID {uid}");
        return;
    };

    log::debug!("Message from USer ID {}: {:?}", uid, msg);


    if let Err(e) = handler.write().await.handle(msg, uid, data).await {
        log::warn!("Command Handler had an error with user ({uid}): {}", e);
        send_err_to_user(uid, data, e).await;
    }
}

pub async fn send_err_to_user(uid: usize, data: &WsServerDataContainer<ServerData>, err: HandlerError) {
    let users = &data.read().await.users;

    let Some(user) = users.get(&uid) else {
        log::error!("Failed to get User ID {uid}");
        return;
    };

    let msg = WsMessage{
        typ: WsMessageType::Error {
            msg: err.msg,
            id: err.id
        },
        authorization: None,
    };

    let Ok(msg) = bincode::serialize(&msg) else {
        log::error!("Failed to serialize message");
        return;
    };

    if let Err(e) = user.ws.send(Message::binary(msg)) {
        log::error!("Failed to send error message to User ID {uid}: {e}");
        return;
    }
}
