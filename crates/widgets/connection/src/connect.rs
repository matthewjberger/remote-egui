use crate::WidgetClient;
use widget::{
    egui,
    serde::{Deserialize, Serialize},
};
use widget::{Broker, Widget};

#[derive(Default, Serialize, Deserialize)]
#[serde(crate = "widget::serde")]
pub struct Connection {
    connection: ConnectionPanel,
}

impl Widget for Connection {
    fn title(&self) -> String {
        "Connection".to_string()
    }

    fn ui(&mut self, ui: &mut egui::Ui, broker: &mut Broker) {
        self.connection.ui(ui, broker);

        // Incoming messages can be ignored but still need
        // to be dequeued for newer messages to be processed
        self.connection.client_mut().next_message();
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "widget::serde")]
pub struct ConnectionPanel {
    broker_address: String,
    client: WidgetClient,
}

impl Default for ConnectionPanel {
    fn default() -> Self {
        Self {
            broker_address: "0.0.0.0:9000".to_string(),
            client: WidgetClient::default(),
        }
    }
}

impl ConnectionPanel {
    pub fn client(&self) -> &WidgetClient {
        &self.client
    }

    pub fn client_mut(&mut self) -> &mut WidgetClient {
        &mut self.client
    }

    pub fn update(&mut self, broker: &mut Broker) {
        self.client.update(broker);
    }
}

impl Widget for ConnectionPanel {
    fn title(&self) -> String {
        "Connection".to_string()
    }

    fn ui(&mut self, _ui: &mut egui::Ui, broker: &mut Broker) {
        self.update(broker);
    }
}
