use connection::ConnectionPanel;
use widget::{
    egui,
    filesystem::FileSystemMessage,
    log,
    serde::{Deserialize, Serialize},
    Broker, Widget,
};

#[derive(Default, Serialize, Deserialize)]
#[serde(crate = "widget::serde")]
pub struct Template {
    connection: ConnectionPanel,
}

impl Widget for Template {
    fn title(&self) -> String {
        "Template".to_string()
    }

    fn ui(&mut self, ui: &mut egui::Ui, broker: &mut Broker) {
        self.connection.ui(ui, broker);

        ui.group(|ui| {
            self.widget_ui(ui, broker);
        });

        self.receive_messages();
    }
}

impl Template {
    fn widget_ui(&mut self, ui: &mut egui::Ui, broker: &mut Broker) {
        let client = self.connection.client_mut();

        if ui.button("Publish RPC command").clicked() {
            client.publish_rpc_command(broker, widget::rpc::Command::Example);
            client.notify(broker, "Published RPC command!");
        }
    }

    fn receive_messages(&mut self) {
        if let Some(FileSystemMessage::File { bytes, .. }) =
            self.connection.client_mut().next_filesystem_message()
        {
            log::debug!("Received file bytes: {} bytes", bytes.len());
        }
    }
}
