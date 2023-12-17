use enum2str::EnumStr;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, EnumStr)]
pub enum EditorLanguage {
    #[enum2str("json")]
    #[default]
    Json,
}

#[derive(Serialize, Deserialize)]
pub struct CodeEditor {
    language: String,
    read_only: bool,
}

impl Default for CodeEditor {
    fn default() -> Self {
        Self {
            language: EditorLanguage::Json.to_string(),
            read_only: false,
        }
    }
}

impl CodeEditor {
    pub fn ui(&mut self, ui: &mut egui::Ui, code: &mut String) -> egui::Response {
        let Self {
            language,
            read_only,
        } = self;

        let theme = crate::CodeTheme::from_memory(ui.ctx());

        let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
            let mut layout_job = crate::highlight(ui.ctx(), &theme, string, language);
            layout_job.wrap.max_width = wrap_width;
            ui.fonts(|f| f.layout_job(layout_job))
        };

        let text = code.to_string();
        let response = ui.add(
            egui::TextEdit::multiline(code)
                .font(egui::TextStyle::Monospace) // for cursor height
                .code_editor()
                .desired_rows(10)
                .lock_focus(true)
                .desired_width(f32::INFINITY)
                .layouter(&mut layouter),
        );
        if *read_only {
            *code = text;
        }
        response
    }

    pub fn set_language(&mut self, language: EditorLanguage) {
        self.language = language.to_string();
    }

    pub fn set_read_only(&mut self, read_only: bool) {
        self.read_only = read_only;
    }
}
