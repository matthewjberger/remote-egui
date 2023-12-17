use crate::{project::Project, tree::TreeBehavior};
use egui::{Button, Visuals};
use enum2pos::EnumIndex;
use enum2str::EnumStr;
use rpc::{Command, RpcResult};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use ui::contract::{
    filesystem::{FileSystemMessage, FileSystemResult},
    Message,
};

#[allow(unused_variables)]
#[derive(Default, EnumStr, EnumIndex)]
enum Tab {
    #[default]
    Editor,
}

macro_rules! match_tab {
    ($tab:expr, $($pattern:ident => $action:block),+ $(,)?) => {
        match $tab {
            $(
                _ if $tab == Tab::$pattern.to_index() => $action,
            )+
            _ => {}
        }
    };
}

#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub struct App {
    url: String,
    command: Command,
    last_result: Option<RpcResult>,
    theme: Theme,
    connection_strategy: BackendConnectionStrategy,

    recents: HashSet<RecentEntry>,

    #[serde(skip)]
    selected_tab: usize,

    #[serde(skip)]
    show_connection_window: bool,

    #[serde(skip)]
    project: Project,

    #[serde(skip)]
    show_side_panel: bool,
}

impl App {
    const URL_BAR_WIDTH: f32 = 100.0;

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn rpc(&self) -> &crate::rpc::Rpc {
        &self.project.behavior.rpc
    }

    pub fn rpc_mut(&mut self) -> &mut crate::rpc::Rpc {
        &mut self.project.behavior.rpc
    }

    fn show_tabs(&mut self, ui: &mut egui::Ui) {
        [Tab::Editor].iter().enumerate().for_each(|(index, label)| {
            let selected = self.selected_tab == index;
            if ui.selectable_label(selected, label.to_string()).clicked() {
                self.selected_tab = index;
            }
        });
    }

    fn menu_bar_ui(
        &mut self,
        ui: &mut egui::Ui,
        frame: &mut eframe::Frame,
        context: &egui::Context,
    ) {
        egui::menu::bar(ui, |ui| {
            self.show_tabs(ui);
            ui.separator();

            match_tab!(self.selected_tab,
                Editor => {
                    self.editor_tab_ui(ui, context, frame);
                },
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if cfg!(debug_assertions) {
                    egui::warn_if_debug_build(ui);
                }

                ui.separator();
            });
        });
    }

    fn connection_ui(&mut self, ui: &mut egui::Ui, context: &egui::Context) {
        ui.group(|ui| {
            // The internal rpc server requires multithreading and is unavailable on wasm
            #[cfg(not(target_arch = "wasm32"))]
            {
                if ui
                    .radio_value(
                        &mut self.connection_strategy,
                        BackendConnectionStrategy::Internal,
                        "Internal",
                    )
                    .changed()
                {
                    let strategy = self.connection_strategy;
                    self.rpc_mut().set_connection_strategy(&strategy);
                }
            }

            self.backend_strategy_ui(ui, context);

            let label = if self.project.behavior.rpc.has_connected() {
                "Backend Available ✅"
            } else {
                "Backend Unavailable ❌"
            }
            .to_string();
            ui.label(label);
        });
    }

    fn backend_strategy_ui(&mut self, ui: &mut egui::Ui, context: &egui::Context) {
        // The remote connections strategy is the only option
        // available in wasm, so it does not need to be explicitly selected
        #[cfg(not(target_arch = "wasm32"))]
        ui.radio_value(
            &mut self.connection_strategy,
            BackendConnectionStrategy::Remote,
            "Remote",
        );

        ui.horizontal(|ui| {
            ui.horizontal(|ui| {
                ui.label("URL:");
                egui::TextEdit::singleline(&mut self.url)
                    .desired_width(Self::URL_BAR_WIDTH)
                    .show(ui);
                let connect_button = ui.add_enabled(
                    matches!(self.connection_strategy, BackendConnectionStrategy::Remote),
                    Button::new("Connect"),
                );
                let enter_key_pressed = ui.input(|i| i.key_pressed(egui::Key::Enter));

                if enter_key_pressed || connect_button.clicked() {
                    let context = context.clone();
                    let wakeup = move || context.request_repaint(); // wake up UI thread on new message
                    self.project
                        .behavior
                        .rpc
                        .connect(&format!("ws://{}", &self.url), wakeup);
                }
            });

            let connected = self.project.behavior.rpc.has_connected();
            let status = if connected {
                "Connected"
            } else {
                "Not Connected"
            };
            ui.label(status);
        });
    }

    fn editor_tab_ui(
        &mut self,
        ui: &mut egui::Ui,
        context: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        self.file_tab_ui(ui);
        self.connection_tab_ui(ui);
        self.theme_tab_ui(ui, context);

        // zoom controls are only available on native
        #[cfg(not(target_arch = "wasm32"))]
        zoom_ui(ui, _frame);

        ui.separator();

        // TODO: enable custom side panels per app
        // ui.checkbox(&mut self.show_side_panel, "⚒")
        //     .on_hover_text("Toggle toolbox");

        ui.checkbox(
            &mut self
                .project
                .behavior
                .simplification_options
                .all_panes_must_have_tabs,
            "👁",
        )
        .on_hover_text("Show/Hide tab bars\n\nAlt + T");

        ui.separator();
    }

    fn file_tab_ui(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Project", |ui| {
            self.open_project_menu_ui(ui);
            self.save_as_menu_ui(ui);

            #[cfg(not(target_arch = "wasm32"))]
            {
                // The recents tab is unavailable on wasm
                // because files are returned as bytes
                // and not as a path like on native
                ui.separator();
                self.recents_tab_ui(ui);
            }
        });
    }

    fn connection_tab_ui(&mut self, ui: &mut egui::Ui) {
        if ui.button("Connection").clicked() {
            self.toggle_connection_window();
        }
    }

    fn open_project_menu_ui(&mut self, ui: &mut egui::Ui) {
        if ui.button("Open").clicked() {
            self.project.pick_file(
                "My app name Project",
                &[],
                &FileSystemTag::Project.to_string(),
            );
            ui.close_menu();
        }
    }

    fn save_as_menu_ui(&mut self, ui: &mut egui::Ui) {
        if ui.button("Save as").clicked() {
            if let Ok(bytes) = self.project.to_bytes() {
                self.project.save_file(&bytes);
            }
            ui.close_menu();
        }
    }

    fn theme_ui(&mut self, ui: &mut egui::Ui, context: &egui::Context) {
        if ui.button("Light").clicked() {
            self.set_theme(context, Theme::Light);
        }

        if ui.button("Dark").clicked() {
            self.set_theme(context, Theme::Dark);
        }
    }

    fn set_theme(&mut self, context: &egui::Context, theme: Theme) {
        self.theme = theme;
        self.reload_theme(context);
    }

    // perf: This might be slow to call each frame
    fn reload_theme(&mut self, context: &egui::Context) {
        let visuals = match &self.theme {
            Theme::Light => Visuals::light(),
            Theme::Dark => Visuals::dark(),
        };
        context.set_visuals(visuals);
    }

    fn theme_tab_ui(&mut self, ui: &mut egui::Ui, context: &egui::Context) {
        ui.menu_button("Theme", |ui| {
            self.theme_ui(ui, context);
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn recents_tab_ui(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Recent projects", |ui| {
            let mut path_to_load = None;
            for RecentEntry { name, path } in self.recents.iter() {
                let label = format!("{name}\n\n{path}");
                if ui.button(label).clicked() {
                    path_to_load = Some(path.to_string());
                }
                ui.separator();
            }
            if let Some(path) = path_to_load.take() {
                self.load_project_from_file(path);
            }
        });
    }

    fn toggle_connection_window(&mut self) {
        self.show_connection_window = !self.show_connection_window;
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn load_project_from_file(&mut self, path: impl AsRef<std::path::Path>) {
        let title = self.project.title.to_string();
        self.add_recent_project(&title, &path);
        log::debug!("Loading project from file: {}", path.as_ref().display());
        let project_bytes = match std::fs::read(&path) {
            Ok(project_string) => project_string,
            Err(error) => {
                log::error!("{error}");
                return;
            }
        };
        self.load_project_from_bytes(&project_bytes);
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn add_recent_project(&mut self, name: &str, path: &impl AsRef<std::path::Path>) {
        self.recents
            .insert(RecentEntry::new(name, &path.as_ref().display().to_string()));
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn load_project_from_bytes(&mut self, project_bytes: &[u8]) {
        if let Ok(project) = Project::from_bytes(project_bytes) {
            self.project = project;
        }
    }

    fn settings_window_ui(&mut self, context: &egui::Context) {
        if !self.show_connection_window {
            return;
        }

        let mut show_connection_window = self.show_connection_window;
        egui::Window::new("Connection")
            .open(&mut show_connection_window)
            .show(context, |ui| {
                ui.group(|ui| {
                    ui.heading("Backend");
                    self.connection_ui(ui, context);
                });
            });
        self.show_connection_window = show_connection_window;
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, context: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("server").show(context, |ui| {
            self.menu_bar_ui(ui, frame, context);
        });

        if self.show_side_panel {
            match_tab!(self.selected_tab,
                Editor => {
                    egui::SidePanel::left("left_side_panel").show(context, |ui| {
                        ui.collapsing("Widgets", |ui| {
                            ui.label("Widget name #1");
                            ui.label("Widget name #2");
                            ui.label("Widget name #3");
                        });
                    });
                },
            );
        }

        egui::CentralPanel::default().show(context, |ui| {
            match_tab!(self.selected_tab,
                Editor => {
                    self.project.update();
                    self.project.ui(ui);
                },
            );
        });

        let TreeBehavior {
            notification_client,
            broker,
            ..
        } = &mut self.project.behavior;
        notification_client.ui(broker, context);

        self.settings_window_ui(context);

        if let Some(Message::FileSystemResult {
            result: FileSystemResult::Success(FileSystemMessage::File { bytes, path, .. }),
        }) = self.project.widget_client_mut().next_message()
        {
            if let Ok(project) = Project::from_bytes(&bytes) {
                self.project = project;
                self.recents
                    .insert(RecentEntry::new(&self.project.title, &path));
            }
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub enum Theme {
    Light,

    #[default]
    Dark,
}

#[cfg(not(target_arch = "wasm32"))]
fn zoom_ui(ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    ui.menu_button("Zoom", |ui| {
        egui::gui_zoom::zoom_menu_buttons(ui, frame.info().native_pixels_per_point);
    });
}

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecentEntry {
    name: String,
    path: String,
}

impl RecentEntry {
    pub fn new(name: &str, path: &str) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Copy, Clone)]
pub enum BackendConnectionStrategy {
    /// Unavailable on Wasm because the backend RPC server requires multithreading
    #[cfg(not(target_arch = "wasm32"))]
    Internal,

    Remote,
}

#[cfg(not(target_arch = "wasm32"))]
impl Default for BackendConnectionStrategy {
    fn default() -> Self {
        Self::Internal
    }
}

#[cfg(target_arch = "wasm32")]
impl Default for BackendConnectionStrategy {
    fn default() -> Self {
        Self::Remote
    }
}

#[derive(Default, EnumStr)]
pub enum FileSystemTag {
    #[default]
    Project,
}
