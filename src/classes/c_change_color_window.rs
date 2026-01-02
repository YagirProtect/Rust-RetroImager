use egui::Context;
use crate::classes::c_dithered_image::DitheredImage;
use crate::classes::c_palette_menu::PaletteMenu;
use crate::classes::c_rgb16::Rgb16;
use crate::classes::t_widget::UIWidget;
use crate::image_utils::{color32_to_rgb16, rgb16_to_color32};

#[derive(Default)]
pub struct ColorReplaceWindow{
    pub is_open: bool,
    pub last_command: ColorReplaceCommand,
    pub start_color: Rgb16,
    pub end_color: Rgb16,
    pub palette_id: usize,
    pub previewed: bool
}
pub enum ColorReplaceCommand{
    None,
    Replace,
    Cancel,
    Preview,
    Reset
}

impl Default for ColorReplaceCommand{
    fn default()->Self{
        ColorReplaceCommand::Replace
    }
}

impl UIWidget for ColorReplaceWindow {
    fn update(&mut self, ctx: &Context) {
        if (self.is_open){
            egui::Window::new("Swap Color".to_string())
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui|{
                    ui.label("Replace selected palette color with:");

                    // Текущий выбранный цвет (как Color32)
                    let mut new_c32 = rgb16_to_color32(self.end_color);

                    // Кнопка с color picker'ом
                    ui.color_edit_button_srgba(&mut new_c32);


                    self.end_color = color32_to_rgb16(new_c32);

                    ui.separator();

                    ui.horizontal(|ui| {
                        if ui.button("Replace").clicked() {
                            self.last_command = ColorReplaceCommand::Replace;
                        }
                        if ui.button("Preview").clicked() {
                            self.previewed = true;
                            self.last_command = ColorReplaceCommand::Preview;
                        }
                        if ui.button("Cancel").clicked() {
                            self.last_command = ColorReplaceCommand::Cancel;
                        }
                        if ui.button("Reset").clicked() {
                            self.last_command = ColorReplaceCommand::Reset;
                        }
                    });
                });
        }
    }
}

impl ColorReplaceWindow {
    pub fn update_color_window(&mut self, ctx: &egui::Context){
        self.update(ctx)
    }


    pub fn open_window(&mut self, palette_id: usize, dithered_image: &DitheredImage) {
        let data = dithered_image.get_palette_colors();
        self.previewed = false;
        self.start_color = data[palette_id].clone();
        self.end_color = data[palette_id].clone();
        self.palette_id = palette_id;

        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.clear_command()
    }

    pub fn clear_command(&mut self){
        self.last_command = ColorReplaceCommand::None;
    }
}