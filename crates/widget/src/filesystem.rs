use enum2str::EnumStr;
use serde::{Deserialize, Serialize};

#[cfg(feature = "gui")]
use enum2egui::{egui, Gui, GuiInspect};

#[derive(Debug, Default, Serialize, Deserialize, EnumStr, Clone, PartialEq, Eq)]
pub enum FileSystemCommand {
    #[default]
    None,

    PickFile {
        tag: String,
        filter_name: String,
        extensions: Vec<String>,
    },

    PickFolder {
        tag: String,
    },

    SaveFile {
        bytes: Vec<u8>,
    },
}

// TODO: Implement UI for this
#[cfg(feature = "gui")]
impl GuiInspect for FileSystemCommand {
    fn ui(&self, _ui: &mut egui::Ui) {}
    fn ui_mut(&mut self, _ui: &mut egui::Ui) {}
}

#[derive(Debug, Serialize, Deserialize, EnumStr, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "gui", derive(Gui))]
pub enum FileSystemResult {
    Success(FileSystemMessage),
    Error(FilesystemError),
}

impl Default for FileSystemResult {
    fn default() -> Self {
        Self::Success(FileSystemMessage::Empty)
    }
}

#[derive(Debug, Default, Serialize, Deserialize, EnumStr, Clone, PartialEq, Eq)]
pub enum FileSystemMessage {
    #[default]
    Empty,

    File {
        path: String,
        bytes: Vec<u8>,
        tag: FileSystemTag,
    },

    Folder {
        path: String,
        tag: FileSystemTag,
    },
}

// TODO: Implement UI for this
#[cfg(feature = "gui")]
impl GuiInspect for FileSystemMessage {
    fn ui(&self, _ui: &mut egui::Ui) {}
    fn ui_mut(&mut self, _ui: &mut egui::Ui) {}
}

#[derive(Default, Debug, Serialize, Deserialize, EnumStr, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "gui", derive(Gui))]
pub enum FilesystemError {
    #[default]
    None,
}

pub type FileSystemId = String;
pub type FileSystemPath = String;
pub type FileSystemBytes = Vec<u8>;
pub type FileSystemTag = String;
