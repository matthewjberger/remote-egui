use template::Template;
use widget::Widget;

macro_rules! impl_widget_enums {
    ($($enum_variant:ident($widget_type:ident) => $enum_str:expr),* $(,)?) => {
        use enum2egui::Gui;
        use serde::{Serialize, Deserialize};
        use enum2str::EnumStr;

        #[derive(EnumStr, Gui, Serialize, Deserialize)]
        pub enum UiWidgetLabel {
            $(
                #[enum2str($enum_str)]
                $enum_variant,
            )*
        }

        #[derive(Serialize, Deserialize, enum2str::EnumStr)]
        pub enum UiWidget {
            $(
                #[enum2str($enum_str)]
                $enum_variant($widget_type),
            )*
        }

        impl Widget for UiWidget {
            fn title(&self) -> String {
                match self {
                    $( UiWidget::$enum_variant(widget) => widget.title(), )*
                }
            }

            fn ui(&mut self, ui: &mut egui::Ui, broker: &mut crate::contract::Broker) {
                match self {
                    $( UiWidget::$enum_variant(widget) => widget.ui(ui, broker), )*
                }
            }
        }

        impl From<&UiWidgetLabel> for UiWidget {
            fn from(label: &UiWidgetLabel) -> Self {
                match label {
                    $( UiWidgetLabel::$enum_variant => UiWidget::$enum_variant($widget_type::default()), )*
                }
            }
        }
    }
}

impl Default for UiWidget {
    fn default() -> Self {
        Self::Template(Template::default())
    }
}

impl Default for UiWidgetLabel {
    fn default() -> Self {
        Self::Template
    }
}

impl_widget_enums!(
    Template(Template) => "Template",
);
