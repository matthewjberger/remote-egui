use egui_tiles::UiResponse;
use enum2egui::GuiInspect;
use enum2str::EnumStr;
use serde::{Deserialize, Serialize};
use ui::{
    contract::{Broker, Widget},
    widgets::{UiWidget, UiWidgetLabel},
};

#[derive(Serialize, Deserialize, EnumStr)]
pub enum Pane {
    #[enum2str("Widget")]
    Widget {
        label: UiWidgetLabel,
        widget: UiWidget,
    },
}

impl Default for Pane {
    fn default() -> Self {
        Self::Widget {
            label: UiWidgetLabel::default(),
            widget: UiWidget::default(),
        }
    }
}

impl Pane {
    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        broker: &mut Broker,
        show_widget_settings: bool,
    ) -> UiResponse {
        match self {
            Pane::Widget { label, widget } => {
                widget_pane_ui(ui, label, widget, broker, show_widget_settings)
            }
        }

        check_for_drag(ui)
    }

    pub fn title(&self) -> String {
        "Empty".to_string()
    }
}

fn widget_pane_ui(
    ui: &mut egui::Ui,
    label: &mut UiWidgetLabel,
    widget: &mut UiWidget,
    broker: &mut Broker,
    show_widget_settings: bool,
) {
    if show_widget_settings {
        ui.group(|ui| {
            widget_settings_ui(ui, label, widget);
        });
    }

    widget.ui(ui, broker);
}

fn widget_settings_ui(ui: &mut egui::Ui, label: &mut UiWidgetLabel, widget: &mut UiWidget) {
    ui.horizontal(|ui| {
        label.ui_mut(ui);
        if ui.button("Assign").clicked() {
            *widget = UiWidget::from(&*label);
        }
    });
}

fn check_for_drag(ui: &mut egui::Ui) -> UiResponse {
    let shift_held = ui.input(|input| input.modifiers.shift_only());
    if !shift_held {
        return UiResponse::None;
    }

    let dragged = ui
        .allocate_rect(ui.clip_rect(), egui::Sense::drag())
        .on_hover_cursor(egui::CursorIcon::Grab)
        .dragged();
    if !dragged {
        return UiResponse::None;
    }

    UiResponse::DragStarted
}
