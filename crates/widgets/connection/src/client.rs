use uuid::Uuid;
use widget::{
    broker::{self, Client},
    filesystem::{FileSystemCommand, FileSystemMessage, FileSystemResult},
    log,
    rpc::{Command, Id, RpcMessage, RpcResult},
    serde::{Deserialize, Serialize},
    ClientHandle, Message,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "widget::serde")]
pub struct WidgetClient {
    frontend_id: Id,
    client_id: Option<Id>,
    subscribed: bool,
    #[serde(skip)]
    handle: ClientHandle,
}

impl Default for WidgetClient {
    fn default() -> Self {
        Self {
            handle: Client::new(),
            frontend_id: Uuid::new_v4().to_string(),
            client_id: None,
            subscribed: false,
        }
    }
}

impl WidgetClient {
    pub fn update(&mut self, broker: &mut broker::Broker<Message>) {
        if !self.subscribed {
            self.create_subscriptions(broker);
        }

        if let Some(Message::RpcResult {
            result: RpcResult::Success(RpcMessage::ClientId { id: client_id }),
        }) = self.peek_message()
        {
            self.client_id = Some(client_id);
        }
    }

    fn create_subscriptions(&mut self, broker: &mut broker::Broker<Message>) {
        self.subscribe_to_topic(&Message::rpc_result_topic(&self.frontend_id), broker);
        self.subscribe_to_topic(
            &Message::file_system_result_topic(&self.frontend_id),
            broker,
        );
        self.subscribed = true;
    }

    fn subscribe_to_topic(&mut self, topic: &str, broker: &mut broker::Broker<Message>) {
        log::debug!("Subscribing to {topic}",);
        broker.subscribe(topic, &self.handle);
    }

    pub fn id(&self) -> Option<&String> {
        self.client_id.as_ref()
    }

    pub fn next_message(&mut self) -> Option<Message> {
        self.handle.borrow_mut().next_message()
    }

    pub fn peek_message(&mut self) -> Option<Message> {
        self.handle.borrow_mut().peek_message()
    }

    pub fn next_rpc_message(&mut self) -> Option<RpcMessage> {
        if let Some(Message::RpcResult {
            result: RpcResult::Success(value),
        }) = self.peek_message()
        {
            log::debug!("Received RPC value: {value:#?}");
            self.next_message(); // Dequeue the message we peeked
            Some(value)
        } else {
            None
        }
    }

    pub fn next_filesystem_message(&mut self) -> Option<FileSystemMessage> {
        match self.peek_message() {
            Some(Message::FileSystemResult {
                result: FileSystemResult::Success(value),
            }) => {
                log::debug!("Received file value: {value:#?}");
                self.next_message(); // Dequeue the message we peeked
                Some(value)
            }
            _ => None,
        }
    }

    // TODO: Add alert levels, etc
    pub fn notify(&self, broker: &mut broker::Broker<Message>, text: &str) {
        let message = Message::Notify {
            text: text.to_string(),
        };
        broker.publish(&Message::notify_topic(), message);
    }

    pub fn pick_file(
        &self,
        broker: &mut broker::Broker<Message>,
        filter_name: &str,
        extensions: &[&str],
        tag: &str,
    ) {
        self.publish_file_command(
            broker,
            FileSystemCommand::PickFile {
                tag: tag.to_string(),
                filter_name: filter_name.to_string(),
                extensions: extensions.iter().map(|s| s.to_string()).collect(),
            },
        );
    }

    pub fn pick_directory(&self, broker: &mut broker::Broker<Message>, tag: &str) {
        self.publish_file_command(
            broker,
            FileSystemCommand::PickFolder {
                tag: tag.to_string(),
            },
        );
    }

    pub fn save_file(&self, broker: &mut broker::Broker<Message>, bytes: Vec<u8>) {
        self.publish_file_command(broker, FileSystemCommand::SaveFile { bytes });
    }

    pub fn has_connected(&self) -> bool {
        self.client_id.is_some()
    }

    pub fn publish_rpc_command(&self, broker: &mut broker::Broker<Message>, command: Command) {
        log::info!("Publishing command: {command:#?}");
        let message = Message::RpcCommand {
            id: self.frontend_id.to_string(),
            command,
        };
        broker.publish(&Message::rpc_command_topic(), message);
    }

    pub fn publish_file_command(
        &self,
        broker: &mut broker::Broker<Message>,
        command: FileSystemCommand,
    ) {
        let message = Message::FileSystemCommand {
            id: self.frontend_id.to_string(),
            command,
        };
        broker.publish(&Message::file_system_command_topic(), message);
    }
}
