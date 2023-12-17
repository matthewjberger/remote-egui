use crate::{pane::Pane, tree::TreeBehavior};
use serde::{Deserialize, Serialize};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub title: String,
    pub version: String,
    pub tree: egui_tiles::Tree<Pane>,
    pub behavior: TreeBehavior,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            version: VERSION.to_string(),
            tree: create_default_tree(),
            behavior: TreeBehavior::default(),
        }
    }
}

fn create_default_tree() -> egui_tiles::Tree<Pane> {
    let mut tiles = egui_tiles::Tiles::default();
    let mut tab_tiles = vec![];
    let tab_tile_child = tiles.insert_pane(Pane::default());
    let tab_tile = tiles.insert_tab_tile(vec![tab_tile_child]);
    tab_tiles.push(tab_tile);
    let root = tiles.insert_tab_tile(tab_tiles);
    egui_tiles::Tree::new(root, tiles)
}

impl Project {
    pub fn widget_client(&self) -> &ui::connection::WidgetClient {
        self.behavior.connection().client()
    }

    pub fn widget_client_mut(&mut self) -> &mut ui::connection::WidgetClient {
        self.behavior.connection_mut().client_mut()
    }

    pub fn pick_file(&mut self, filter_name: &str, extensions: &[&str], tag: &str) {
        let TreeBehavior {
            connection, broker, ..
        } = &mut self.behavior;
        connection
            .client_mut()
            .pick_file(broker, filter_name, extensions, tag);
    }

    pub fn save_file(&mut self, bytes: &[u8]) {
        let TreeBehavior {
            connection, broker, ..
        } = &mut self.behavior;
        connection.client_mut().save_file(broker, bytes.to_vec());
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        if self.tree.tiles.iter().count() == 0 {
            self.tree = create_default_tree();
        }

        self.tree.ui(&mut self.behavior, ui);

        if let Some(parent) = self.behavior.add_child_to.take() {
            let new_child = self.tree.tiles.insert_pane(Pane::default());
            if let Some(egui_tiles::Tile::Container(egui_tiles::Container::Tabs(tabs))) =
                self.tree.tiles.get_mut(parent)
            {
                tabs.add_child(new_child);
                tabs.set_active(new_child);
            }
        }

        if let Some(parent) = self.behavior.child_removed.take() {
            if let Some(egui_tiles::Tile::Container(egui_tiles::Container::Tabs(tabs))) =
                self.tree.tiles.get_mut(parent)
            {
                if let Some(active_child) = tabs.active.take() {
                    self.tree.tiles.remove_recursively(active_child);
                }
            }
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(&self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize::<Self>(bytes)
    }

    pub fn update(&mut self) {
        self.behavior.update();
    }
}
