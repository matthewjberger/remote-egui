#[cfg(target_arch = "wasm32")]
use crate::frontend::FrontEnd;

use serde::{Deserialize, Serialize};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Default, Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    url: String,
    error: String,

    #[cfg(target_arch = "wasm32")]
    #[serde(skip)]
    frontend: Option<FrontEnd>,
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    #[cfg(target_arch = "wasm32")]
    fn connect(&mut self, ctx: egui::Context) {
        let wakeup = move || ctx.request_repaint(); // wake up UI thread on new message
        match ewebsock::connect_with_wakeup(&self.url, wakeup) {
            Ok((ws_sender, ws_receiver)) => {
                self.frontend = Some(FrontEnd::new(ws_sender, ws_receiver));
                self.error.clear();
            }
            Err(error) => {
                log::error!("Failed to connect to {:?}: {}", &self.url, error);
                self.error = error;
            }
        }
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}

    #[cfg(target_arch = "wasm32")]
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("server").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("URL:");
                if ui.text_edit_singleline(&mut self.url).lost_focus()
                    && ui.input(|i| i.key_pressed(egui::Key::Enter))
                {
                    self.connect(ctx.clone());
                }
            });
        });

        if !self.error.is_empty() {
            egui::TopBottomPanel::top("error").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Error:");
                    ui.colored_label(egui::Color32::RED, &self.error);
                });
            });
        }

        if let Some(frontend) = &mut self.frontend {
            frontend.ui(ctx);
        }
    }
}
