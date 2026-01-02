use eframe::epaint::StrokeKind;
use egui::{Color32, Image};
use crate::classes::c_dithered_image::DitheredImage;
use crate::classes::c_rgb16::Rgb16;
use crate::classes::t_widget::UIWidget;
use crate::image_utils::u16_to_u8;

#[derive(Default)]
pub struct PaletteMenu{
    last_palette: Vec<Rgb16>,
    is_enabled: bool,
    pub selected: Option<usize>,
    pub last_command: PaletteMenuCommand
}

#[derive(Default)]
pub enum PaletteMenuCommand{
    #[default]
    None,
    OpenPaletteWindow
}

impl PaletteMenu {

    pub fn clear_selection(&mut self) {
        self.selected = None;
        self.last_command = PaletteMenuCommand::None;
    }

    pub fn update_palette(&mut self, ctx: &egui::Context, dithered_image: &DitheredImage, is_enabled: bool) {

        self.last_palette = dithered_image.get_palette_colors().clone();
        self.is_enabled = is_enabled;

        self.update(ctx);
    }
}

impl UIWidget for PaletteMenu {
    fn update(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("PaletteMenu").show(ctx, |ui| {
            let swatch = egui::vec2(18.0, 18.0); // размер квадратика
            let rounding = egui::Rounding::same(1.0 as u8);

            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(6.0, 6.0);

                for (i, &c) in self.last_palette.iter().enumerate() {

                    let (rect, resp) = ui.allocate_exact_size(swatch, egui::Sense::click());


                    let mut strokeColor = egui::Color32::from_gray(80);


                    let mut width = 1.0;

                    if (self.is_enabled) {
                        if (resp.hovered()) {
                            strokeColor = egui::Color32::from_rgb(150, 150, 150);
                            width = 2.0;
                        }
                        if (resp.clicked()) {
                            self.selected = Some(i);
                            
                            
                            self.last_command = PaletteMenuCommand::OpenPaletteWindow;
                        }
                        if (self.selected.is_some()) {
                            if (i == self.selected.unwrap()) {
                                strokeColor = egui::Color32::from_rgb(230, 230, 230);
                                width = 4.0
                            }
                        }
                    }

                    let col = Color32::from_rgb(u16_to_u8(c.r), u16_to_u8(c.g), u16_to_u8(c.b));
                    // заливка
                    ui.painter().rect_filled(rect, rounding, col);

                    // обводка (чтобы на тёмном фоне было видно)
                    ui.painter().rect_stroke(
                        rect,
                        rounding,
                        egui::Stroke::new(width, strokeColor),
                        StrokeKind::Inside
                    );
                }
            });
        });
    }
}

