use broker::Client;
use ui::contract::{
    filesystem::{FileSystemCommand, FileSystemMessage, FileSystemResult},
    Broker, ClientHandle, Message,
};

#[cfg(target_arch = "wasm32")]
use ui::contract::filesystem::{FileSystemBytes, FileSystemId, FileSystemTag};

#[cfg(target_arch = "wasm32")]
use futures::channel::oneshot::{self, Receiver};

pub type Id = String;

pub struct FileSystemClient {
    client: ClientHandle,
    subscribed: bool,

    #[cfg(target_arch = "wasm32")]
    file_receiver: Option<Receiver<(FileSystemId, FileSystemBytes, FileSystemTag)>>,
}

impl Default for FileSystemClient {
    fn default() -> Self {
        Self {
            subscribed: false,
            client: Client::with_ring_buffer_size(100),
            #[cfg(target_arch = "wasm32")]
            file_receiver: None,
        }
    }
}

impl FileSystemClient {
    pub fn new() -> Self {
        Self::default()
    }

    fn subscribe(&mut self, broker: &mut Broker) {
        broker.subscribe(&Message::file_system_command_topic(), &self.client);
        self.subscribed = true;
    }

    pub fn update(&mut self, broker: &mut Broker) {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(file_receiver) = self.file_receiver.as_mut() {
                if let Ok(Some((id, bytes, tag))) = file_receiver.try_recv() {
                    // The path is not returned to us in wasm, only the file bytes
                    let path = "".to_string();
                    publish_system_result(broker, &id, &path, bytes, &tag);
                }
            }
        }

        if !self.subscribed {
            self.subscribe(broker);
        }

        let mut messages = Vec::new();
        while let Some(message) = self.client.borrow_mut().next_message() {
            messages.push(message);
        }

        messages.into_iter().for_each(|message| {
            if let Message::FileSystemCommand { id, command } = &message {
                self.execute(broker, id, command)
            }
        });
    }

    fn execute(&mut self, broker: &mut Broker, id: &Id, command: &FileSystemCommand) {
        match command {
            FileSystemCommand::PickFile {
                filter_name,
                extensions,
                tag,
            } => self.pick_file(broker, id, filter_name, extensions, tag),
            FileSystemCommand::SaveFile { bytes } => self.save_file(bytes),
            #[cfg(not(target_arch = "wasm32"))]
            FileSystemCommand::PickFolder { tag } => self.pick_folder(broker, id, tag),
            _ => {}
        }
    }

    fn pick_file(
        &mut self,
        _broker: &mut Broker,
        id: &Id,
        filter_name: &str,
        extensions: &[String],
        tag: &str,
    ) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter(filter_name, extensions)
                .pick_file()
            {
                log::debug!("File picked: {path:#?}");
                let bytes = match std::fs::read(&path) {
                    Ok(bytes) => bytes,
                    Err(error) => {
                        log::error!("{error}");
                        return;
                    }
                };
                publish_system_result(_broker, id, &path.display().to_string(), bytes, tag);
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            // TODO: Use a data structure here instead of a tuple
            let (sender, receiver) =
                oneshot::channel::<(FileSystemId, FileSystemBytes, FileSystemTag)>();
            self.file_receiver = Some(receiver);
            let task = rfd::AsyncFileDialog::new()
                .add_filter(filter_name, extensions)
                .pick_file();
            let id = id.to_string();
            let tag = tag.to_string();
            wasm_bindgen_futures::spawn_local(async {
                let file = task.await;
                if let Some(file) = file {
                    let bytes = file.read().await;
                    let _ = sender.send((id, bytes, tag));
                }
            });
        }
    }

    fn save_file(&mut self, bytes: &[u8]) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            use std::io::Write;
            if let Some(path) = rfd::FileDialog::new().save_file() {
                let path_str = path.display().to_string();
                if let Ok(mut output) = std::fs::File::create(path) {
                    match output.write_all(bytes) {
                        Ok(_) => log::debug!("File saved to {path_str}"),
                        Err(error) => log::debug!("{error}"),
                    }
                }
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let task = rfd::AsyncFileDialog::new().save_file();
            let bytes = bytes.to_vec();
            wasm_bindgen_futures::spawn_local(async move {
                let file = task.await;
                if let (Some(file), bytes) = (file, bytes) {
                    if let Err(error) = file.write(&bytes).await {
                        log::error!("{error}");
                    }
                }
            });
        }
    }

    // Folder picking is not possible yet in wasm
    // https://docs.rs/rfd/latest/rfd/struct.AsyncFileDialog.html#method.pick_folder
    #[cfg(not(target_arch = "wasm32"))]
    fn pick_folder(&mut self, _broker: &mut Broker, id: &Id, tag: &str) {
        if let Some(folder) = rfd::FileDialog::new().pick_folder() {
            log::debug!("Folder picked: {folder:#?}");
            publish_folder_result(_broker, id, folder, tag);
        }
    }
}

fn publish_system_result(
    broker: &mut broker::Broker<Message>,
    id: &str,
    path: &str,
    bytes: Vec<u8>,
    tag: &str,
) {
    broker.publish(
        &Message::file_system_result_topic(id),
        Message::FileSystemResult {
            result: FileSystemResult::Success(FileSystemMessage::File {
                path: path.to_string(),
                bytes,
                tag: tag.to_string(),
            }),
        },
    );
}

#[cfg(not(target_arch = "wasm32"))]
fn publish_folder_result(
    broker: &mut broker::Broker<Message>,
    id: &str,
    folder_path: std::path::PathBuf,
    tag: &str,
) {
    broker.publish(
        &Message::file_system_result_topic(id),
        Message::FileSystemResult {
            result: FileSystemResult::Success(FileSystemMessage::Folder {
                path: folder_path.display().to_string(),
                tag: tag.to_string(),
            }),
        },
    );
}
