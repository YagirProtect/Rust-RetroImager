use std::path::PathBuf;
use eframe::egui;
use crate::classes::t_widget::UIWidget;

pub struct OkDialog {
    pub is_open: bool,
    last_action: Action,

    ok_text: String,
    cancel_text: String,
    header_text: String,
}
#[derive(Copy, Clone)]
pub enum Action {
    None,
    Ok,
    Cancel,
}
impl Default for OkDialog {
    fn default() -> Self {
        Self{
            is_open: false,
            last_action: Action::None,
            ok_text: "Ok".to_string(),
            cancel_text: "Cancel".to_string(),
            header_text: "Header".to_string(),
        }
    }
}

impl UIWidget for OkDialog {
    fn update(&mut self, ctx: &egui::Context) {
        if (self.is_open) {
            egui::Window::new(self.header_text.clone())
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    // ui.label(self.header_text.clone());
                    ui.horizontal(|ui| {
                        if ui.button(self.ok_text.clone()).clicked() {
                            self.last_action = Action::Ok;
                        }
                        if ui.button(self.cancel_text.clone()).clicked() {
                            self.last_action = Action::Cancel;
                        }
                    });
                });
        }
    }
}

impl OkDialog {
    pub fn clear_command(&mut self) {
        self.last_action = Action::None;
    }
    pub fn get_command(&self) -> Action{
        self.last_action.clone()
    }

    pub fn close(&mut self){
        self.is_open = false;
        self.clear_command();
    }
    pub fn open_dialog(&mut self, ok: &str, cancel: &str, header: &str) {
        self.is_open = true;
        self.ok_text = ok.to_string();
        self.cancel_text = cancel.to_string();
        self.header_text = header.to_string();
    }
}