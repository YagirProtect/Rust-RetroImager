use eframe::egui;
use crate::classes::c_dithered_image::DitheredImage;
use crate::classes::t_widget::UIWidget;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopPanelCommands{
    None,
    OpenFile,
    SaveFile,
    Exit,
    OpenConfig
}

#[derive(Default)]
pub struct TopMenu{
    command: TopPanelCommands,
    is_image_loaded: bool,
    is_enabled: bool,
}

impl TopMenu {
    pub fn get_active_command(&self) -> TopPanelCommands {
        return self.command.clone();
    }

    pub fn clear_active_command(&mut self){
        self.command = TopPanelCommands::None;
    }
}

impl Default for TopPanelCommands {
    fn default() -> Self {
        TopPanelCommands::None
    }
}

impl TopMenu {
    pub(crate) fn update_menu(&mut self, ctx: &egui::Context, dithered_image: &DitheredImage, enabled:bool) {
        self.is_image_loaded = dithered_image.has_image();
        self.is_enabled = enabled;

        self.update(ctx);
    }
}
impl UIWidget for TopMenu {
    fn update(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_menu")
            .show(ctx, |ui| {


                ui.set_enabled(self.is_enabled);

                ui.horizontal(|ui| {
                    ui.menu_button("     File     ", |ui| {
                        if (ui.button("Open File...").clicked()) { self.command = TopPanelCommands::OpenFile }

                        ui.scope(|ui| {

                            ui.set_enabled(self.is_image_loaded && self.is_enabled);
                            if (ui.button("Save File...").clicked()) { self.command = TopPanelCommands::SaveFile }
                        });

                        if (ui.button("Exit").clicked()) { self.command = TopPanelCommands::Exit }
                    });

                    if (ui.button("   Config   ").clicked()){
                        self.command = TopPanelCommands::OpenConfig
                    }
                });
            });
    }
}