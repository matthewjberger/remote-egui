use crate::{Command, Id, Message, Response};
use ewebsock::{WsEvent, WsMessage, WsReceiver, WsSender};

pub struct RpcClient {
    sender: WsSender,
    receiver: WsReceiver,
}

impl RpcClient {
    pub fn new(sender: WsSender, receiver: WsReceiver) -> Self {
        Self { sender, receiver }
    }

    pub fn send(&mut self, id: Id, command: Command) {
        log::debug!("Executing command: {command:#?}");
        let message = Message { id, command };
        let message_bytes = match bincode::serialize(&message) {
            Ok(bytes) => bytes,
            Err(error) => {
                log::error!("{error}");
                return;
            }
        };
        self.sender.send(WsMessage::Binary(message_bytes));
    }

    pub fn receive(&mut self) -> Option<Response> {
        while let Some(event) = self.receiver.try_recv() {
            log::trace!("Received websocket event: {event:?}");
            if let WsEvent::Message(WsMessage::Binary(bytes)) = event {
                return match bincode::deserialize::<Response>(&bytes) {
                    Ok(response) => {
                        log::debug!("Received RPC response: {response:#?}");
                        Some(response)
                    }
                    Err(error) => {
                        log::error!("{error}");
                        None
                    }
                };
            };
        }
        None
    }
}
