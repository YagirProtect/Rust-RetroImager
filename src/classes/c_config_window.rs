use crate::classes::c_config::Config;
use crate::classes::c_dithered_image::DitheredImage;
use crate::classes::c_rgb16::Rgb16;
use crate::classes::t_widget::UIWidget;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigWindowCommands{
    None,
    Save,
    Cancel,
    Reload
}

pub struct ConfigWindow{
    pub is_open: bool,
    colors_palette_size: u16,
    image_percent: f32,

    pub is_previewed: bool,
    pub old_colors_palette_size: u16,
    pub old_image_percent: f32,

    pub palette : Vec<Rgb16>,
    pub palette_override : Vec<Rgb16>,


    last_command: ConfigWindowCommands,
}


impl Default for ConfigWindow {
    fn default() -> Self {
        Self{
            is_open: false,
            colors_palette_size: 0,
            image_percent: 0.0,
            is_previewed: false,
            old_colors_palette_size: 0,
            old_image_percent: 0.0,
            palette: vec![],
            palette_override: vec![],
            last_command: ConfigWindowCommands::None,

        }
    }
}

impl ConfigWindow{
    pub fn new(config: &Config) -> Self{
        Self {
            is_open: false,
            colors_palette_size: config.colors_palette_size,
            image_percent: config.image_percent * 100.0,
            is_previewed: false,
            old_colors_palette_size: 0,
            old_image_percent: 0.0,
            palette: vec![],
            palette_override: vec![],
            last_command: ConfigWindowCommands::None,

        }
    }

    pub fn close(&mut self){
        self.is_open = false;
        self.clear_last_command();
    }

    pub fn clear_last_command(&mut self) {
        self.last_command = ConfigWindowCommands::None;
    }
    pub fn open_config_window(&mut self, config: &Config, dithered_image: &DitheredImage){
        self.is_open = true;
        self.colors_palette_size = config.colors_palette_size;
        self.image_percent = config.image_percent * 100.0;

        self.old_colors_palette_size = self.colors_palette_size;
        self.old_image_percent = self.image_percent;

        self.palette = dithered_image.get_pure_palette_colors().clone();
        self.palette_override = dithered_image.get_palette_colors().clone();

        self.is_previewed = false;
    }

    pub fn get_data_cfg(&self) -> (u16, f32){
        (self.colors_palette_size, self.image_percent/100.0)
    }

    pub fn get_active_command(&self) -> ConfigWindowCommands {
        return self.last_command;
    }
}

impl UIWidget for ConfigWindow {
    fn update(&mut self, ctx: &egui::Context) {
        if (self.is_open) {
            egui::Window::new("Configuration Window".to_string())
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Size Percentage: ");
                            ui.add(egui::Slider::new(&mut self.image_percent, 10.0..=100.0));
                        });

                        ui.horizontal(|ui| {
                            ui.label("Colors Count:       ");
                            ui.add(egui::Slider::new(&mut self.colors_palette_size, 2..=32));
                        });

                        ui.horizontal(|ui| {
                            if (ui.button("Save").clicked()) {
                                self.last_command = ConfigWindowCommands::Save;
                            }
                            if (ui.button("Close").clicked()) {
                                self.last_command = ConfigWindowCommands::Cancel;
                            }
                            if (ui.button("Draw").clicked()) {
                                self.is_previewed = true;
                                self.last_command = ConfigWindowCommands::Reload;
                            }
                        })
                    })
                });
        }
    }
}