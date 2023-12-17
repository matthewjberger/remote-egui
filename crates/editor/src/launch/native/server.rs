use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt, TryStreamExt,
};
use log::{error, info};
use rpc::{Id, Message, Response, RpcExecutor};
use std::{sync::Arc, time::Duration};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{RwLock, RwLockWriteGuard},
};
use tokio_tungstenite::{tungstenite::Message as WebsocketMessage, WebSocketStream};

pub(crate) async fn listen(port: u16) {
    let address = format!("0.0.0.0:{port}");

    let try_socket = TcpListener::bind(&address).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {address}");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }
}

async fn accept_connection(stream: TcpStream) {
    let address = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    info!("Peer address: {address}");

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {address}");

    let server = Arc::new(RwLock::new(RpcExecutor));

    let (mut write, mut read) = ws_stream.split();

    loop {
        receive_rpc_messages(&mut read, &server, &mut write).await;
        tokio::time::sleep(Duration::from_millis(1_000)).await;
    }
}

async fn receive_rpc_messages(
    read: &mut SplitStream<WebSocketStream<TcpStream>>,
    server: &Arc<RwLock<RpcExecutor>>,
    write: &mut SplitSink<WebSocketStream<TcpStream>, WebsocketMessage>,
) {
    if let Ok(Some(WebsocketMessage::Binary(bytes))) = read.try_next().await {
        receive_bytes(server, bytes, write).await;
    }
}

async fn receive_bytes(
    server: &Arc<RwLock<RpcExecutor>>,
    bytes: Vec<u8>,
    write: &mut SplitSink<WebSocketStream<TcpStream>, WebsocketMessage>,
) {
    let handle = server.write().await;
    if let Ok(Message { id, command }) = bincode::deserialize::<Message>(&bytes) {
        let response = execute_command(id, handle, command);
        send_response(response, write).await;
    }
}

fn execute_command(
    id: Id,
    mut handle: RwLockWriteGuard<'_, RpcExecutor>,
    command: rpc::Command,
) -> Response {
    info!("[RPC ->]: {command:#?}");
    let result = handle.execute(&id, command);
    Response {
        id: id.to_string(),
        result,
    }
}

async fn send_response(
    response: Response,
    write: &mut SplitSink<WebSocketStream<TcpStream>, WebsocketMessage>,
) {
    info!("[RPC <-]: {response:#?}");
    if let Ok(response_bytes) = bincode::serialize(&response) {
        if let Err(error) = write.send(WebsocketMessage::Binary(response_bytes)).await {
            error!("Failed to send response: {error}")
        }
    }
}
