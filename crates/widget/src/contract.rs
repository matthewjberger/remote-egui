use crate::filesystem::{FileSystemCommand, FileSystemId, FileSystemResult};
use enum2contract::EnumContract;
use enum2str::EnumStr;
use rpc::{Command, Id as RpcId, RpcResult};
use serde::{Deserialize, Serialize};

pub type ClientHandle = crate::broker::ClientHandle<Message>;
pub type Broker = crate::broker::Broker<Message>;

#[cfg(feature = "gui")]
use enum2egui::{egui, Gui, GuiInspect};

#[derive(Default, Debug, EnumContract, Clone, EnumStr, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "gui", derive(Gui))]
pub enum Message {
    #[default]
    #[topic("")]
    Empty,

    #[topic("rpc/command")]
    RpcCommand { id: RpcId, command: Command },

    #[topic("rpc/{id}/result")]
    RpcResult { result: RpcResult },

    #[topic("file/command")]
    FileSystemCommand {
        id: FileSystemId,
        command: FileSystemCommand,
    },

    #[topic("file/{id}/result")]
    FileSystemResult { result: FileSystemResult },

    #[topic("notify")]
    Notify { text: String },
}

pub trait Widget {
    fn title(&self) -> String;
    fn ui(&mut self, _ui: &mut egui::Ui, _broker: &mut Broker);
}
