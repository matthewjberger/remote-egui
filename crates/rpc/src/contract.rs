use enum2str::EnumStr;
use serde::{Deserialize, Serialize};

#[cfg(feature = "gui")]
use enum2egui::{egui, Gui, GuiInspect};

pub type Id = String;
pub type Topic = String;
pub type PayloadBytes = Vec<u8>;
pub type PayloadJson = String;
pub type IpAddress = String;

#[derive(Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Message {
    pub id: Id,
    pub command: Command,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "gui", derive(Gui))]
pub struct Response {
    pub id: Id,
    pub result: RpcResult,
}

#[derive(Debug, Default, Serialize, Deserialize, EnumStr, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "gui", derive(Gui))]
pub enum RpcMessage {
    #[default]
    Empty,

    ClientId {
        id: Id,
    },

    ConnectionStatus {
        connected: bool,
    },
}

#[derive(Debug, Default, Serialize, Deserialize, EnumStr, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "gui", derive(Gui))]
pub enum Command {
    #[default]
    Example,
}

#[derive(Debug, Serialize, Deserialize, EnumStr, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "gui", derive(Gui))]
pub enum RpcResult {
    Success(RpcMessage),
    Error(Error),
}

impl Default for RpcResult {
    fn default() -> Self {
        Self::value(RpcMessage::default())
    }
}

impl RpcResult {
    pub fn value(response: RpcMessage) -> Self {
        Self::Success(response)
    }
}

#[derive(Default, Debug, Serialize, Deserialize, EnumStr, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "gui", derive(Gui))]
pub enum Error {
    #[default]
    Empty,

    #[enum2str("The RPC operation timed out.")]
    Timeout,

    #[enum2str("The client '{id}' has not been created yet.")]
    UnknownClientId { id: Id },

    #[enum2str("The spawner '{id}' has not been created yet.")]
    UnknownSpawnerId { id: Id },

    #[enum2str("The spawner app '{id}' has not been created yet.")]
    UnknownSpawnerApp { id: Id },

    #[enum2str("The client failed to connect.")]
    Connection,

    #[enum2str("The spawner  failed to spawn apps. {error}")]
    Spawner { error: String },

    #[enum2str("Failed to deserialize a result. Error: {error}")]
    RpcResultDeserialization { error: String },

    #[enum2str("An unexpected message was received.")]
    UnrecognizedMessage,

    #[enum2str("Failed to serialize a command. Error: {error}")]
    CommandSerialization { error: String },

    #[enum2str("Subscription to topic '{topic}' with client '{id}' failed. Error: {error}")]
    Subscription { topic: Topic, id: Id, error: String },

    #[enum2str("Publishing to topic '{topic}' with client '{id}' failed. Error: {error}")]
    Publish { topic: Topic, id: Id, error: String },

    #[enum2str("Publishing json to topic '{topic}' with client '{id}' failed. Error: {error}")]
    PublishJson { topic: Topic, id: Id, error: String },

    #[enum2str("Requesting bridge from '{source_address}' to '{target_address}' with client '{id}' failed. Error: {error}")]
    RequestBridge {
        id: String,
        source_address: String,
        target_address: String,
        error: String,
    },

    #[enum2str("Removing bridge to '{target_address}' with client '{id}' failed. Error: {error}")]
    RemoveBridge {
        id: String,
        target_address: String,
        error: String,
    },
}
