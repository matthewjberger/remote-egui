use broker::Client;
use egui_toast::{Toast, ToastKind, ToastOptions};
use ui::contract::{Broker, ClientHandle, Message};

pub type Id = String;

pub struct NotificationClient {
    client: ClientHandle,
    subscribed: bool,
}

impl Default for NotificationClient {
    fn default() -> Self {
        Self {
            subscribed: false,
            client: Client::with_ring_buffer_size(100),
        }
    }
}

impl NotificationClient {
    pub fn new() -> Self {
        Self::default()
    }

    fn subscribe(&mut self, broker: &mut Broker) {
        broker.subscribe(&Message::notify_topic(), &self.client);
        self.subscribed = true;
    }

    pub fn ui(&mut self, broker: &mut Broker, context: &egui::Context) {
        if !self.subscribed {
            self.subscribe(broker);
        }

        let mut messages = Vec::new();
        while let Some(message) = self.client.borrow_mut().next_message() {
            messages.push(message);
        }

        let mut toasts = egui_toast::Toasts::new()
            .anchor(egui::Align2::RIGHT_BOTTOM, (-10.0, -10.0))
            .direction(egui::Direction::BottomUp);

        messages.into_iter().for_each(|message| {
            // TODO: make notifications configurable
            if let Message::Notify { text } = message {
                toasts.add(Toast {
                    text: text.into(),
                    kind: ToastKind::Info,
                    options: ToastOptions::default()
                        .duration_in_seconds(5.0)
                        .show_progress(true),
                });
            }
        });

        toasts.show(context);
    }
}
