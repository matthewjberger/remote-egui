use crate::{filesystem::FileSystemClient, notification::NotificationClient, pane::Pane, rpc::Rpc};
use egui::{Key, KeyboardShortcut, Modifiers};
use egui_tiles::{SimplificationOptions, TileId, UiResponse};
use serde::{Deserialize, Serialize};
use ui::{connection::ConnectionPanel, contract::Broker};

#[derive(Serialize, Deserialize)]
pub struct TreeBehavior {
    #[serde(skip, default = "TreeBehavior::default_simplification_options")]
    pub simplification_options: SimplificationOptions,
    pub tab_bar_height: f32,
    pub gap_width: f32,
    pub add_child_to: Option<TileId>,
    pub child_removed: Option<TileId>,
    pub show_widget_settings: bool,
    pub connection: ConnectionPanel,

    #[serde(skip)]
    pub broker: Broker,

    #[serde(skip)]
    pub rpc: Rpc,

    #[serde(skip)]
    pub file_client: FileSystemClient,

    #[serde(skip)]
    pub notification_client: NotificationClient,
}

impl Default for TreeBehavior {
    fn default() -> Self {
        Self {
            simplification_options: Self::default_simplification_options(),
            tab_bar_height: 24.0,
            gap_width: 2.0,
            add_child_to: None,
            child_removed: None,
            broker: Broker::new(),
            show_widget_settings: true,
            connection: ConnectionPanel::default(),
            rpc: Rpc::default(),
            file_client: FileSystemClient::default(),
            notification_client: NotificationClient::default(),
        }
    }
}

impl egui_tiles::Behavior<Pane> for TreeBehavior {
    fn pane_ui(&mut self, ui: &mut egui::Ui, _tile_id: TileId, pane: &mut Pane) -> UiResponse {
        pane.ui(ui, &mut self.broker, self.show_widget_settings)
    }

    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        pane.title().into()
    }

    fn top_bar_rtl_ui(
        &mut self,
        _tiles: &egui_tiles::Tiles<Pane>,
        ui: &mut egui::Ui,
        tile_id: TileId,
        _tabs: &egui_tiles::Tabs,
    ) {
        if ui.input_mut(|input| input.consume_shortcut(&TreeShortcuts::NEW_TAB)) {
            self.add_child_to = Some(tile_id);
        }

        if ui.button("🗑").clicked() {
            self.child_removed = Some(tile_id);
        }

        if ui.button("➕").clicked() {
            self.add_child_to = Some(tile_id);
        }
    }

    fn tab_bar_height(&self, _style: &egui::Style) -> f32 {
        self.tab_bar_height
    }

    fn gap_width(&self, _style: &egui::Style) -> f32 {
        self.gap_width
    }

    fn simplification_options(&self) -> SimplificationOptions {
        self.simplification_options
    }
}

impl TreeBehavior {
    pub fn connection(&self) -> &ConnectionPanel {
        &self.connection
    }

    pub fn connection_mut(&mut self) -> &mut ConnectionPanel {
        &mut self.connection
    }

    pub fn notification_client(&mut self) -> &mut NotificationClient {
        &mut self.notification_client
    }

    pub fn update(&mut self) {
        self.connection.update(&mut self.broker);
        self.rpc.update(&mut self.broker);
        self.file_client.update(&mut self.broker);
    }

    pub fn toggle_widget_settings(&mut self) {
        self.show_widget_settings = !self.show_widget_settings;
    }

    pub fn toggle_pane_tabs(&mut self) {
        let SimplificationOptions {
            all_panes_must_have_tabs,
            ..
        } = self.simplification_options;
        self.simplification_options.all_panes_must_have_tabs = !all_panes_must_have_tabs;
    }

    fn default_simplification_options() -> egui_tiles::SimplificationOptions {
        egui_tiles::SimplificationOptions {
            all_panes_must_have_tabs: true,
            ..Default::default()
        }
    }
}

struct TreeShortcuts;

impl TreeShortcuts {
    pub const NEW_TAB: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::T);
}
