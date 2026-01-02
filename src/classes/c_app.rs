use std::path::PathBuf;
use eframe::egui;
use egui::Context;
use crate::classes::c_change_color_window::{ColorReplaceCommand, ColorReplaceWindow};
use crate::classes::c_config::Config;
use crate::classes::c_config_window::{ConfigWindow, ConfigWindowCommands};
use crate::classes::c_dithered_image::DitheredImage;
use crate::classes::c_ok_dialog::{Action, OkDialog};
use crate::classes::c_palette_menu::{PaletteMenu, PaletteMenuCommand};
use crate::classes::c_top_panel::{TopMenu, TopPanelCommands};
use crate::classes::t_widget::UIWidget;

#[derive(Default)]
pub struct App {
    config: Config,
    dithered_image: DitheredImage,
    top_menu: TopMenu,
    ok_dialog: OkDialog,
    config_window: ConfigWindow,
    palette_menu: PaletteMenu,
    color_swap: ColorReplaceWindow
}


impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let mut config = Config::default();
        config.read_file();
        Self {
            config_window: ConfigWindow::new(&config),
            dithered_image: DitheredImage::new(&cc.egui_ctx),
            ok_dialog: OkDialog::default(),
            top_menu: TopMenu::default(),
            palette_menu: PaletteMenu::default(),
            color_swap: ColorReplaceWindow::default(),
            config: config,
        }
    }

    fn match_top_panel_commands(&mut self, ctx: &Context){
        match self.top_menu.get_active_command() {
            TopPanelCommands::None => {}
            TopPanelCommands::OpenFile => {
                if (self.dithered_image.has_image()) {
                    self.ok_dialog.open_dialog("Rewrite image?", "Cancel", "Open new image?");
                    match self.ok_dialog.get_command() {
                        Action::Ok => {
                            //Open file picker
                            self.open_image_picker(ctx);
                            self.ok_dialog.close();
                            self.palette_menu.clear_selection();
                            self.top_menu.clear_active_command();
                        }
                        Action::Cancel => {
                            self.ok_dialog.close();
                            self.top_menu.clear_active_command();
                        }
                        _=>{}
                    }
                }else{
                    self.open_image_picker(ctx);
                    self.top_menu.clear_active_command();
                }
            }
            TopPanelCommands::SaveFile => {

                self.open_save_file(ctx);
                self.top_menu.clear_active_command();
            }
            TopPanelCommands::Exit => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            TopPanelCommands::OpenConfig => {
                self.config_window.open_config_window(&self.config, &self.dithered_image);
                self.top_menu.clear_active_command();
            }
        };

        self.ok_dialog.clear_command();
    }

    pub fn match_config_window_commands(&mut self, ctx: &Context) {
        match self.config_window.get_active_command() {
            ConfigWindowCommands::None => {}
            ConfigWindowCommands::Save => {

                let (colors_count, size) = self.config_window.get_data_cfg();

                self.config.set_colors_count(colors_count);
                self.config.set_size(size);

                self.config.write_file();

                self.config_window.close();

                self.dithered_image.reload(ctx, &self.config);
                self.palette_menu.clear_selection();
            }
            ConfigWindowCommands::Cancel => {

                let has_change = 
                    self.config.colors_palette_size != self.config_window.old_colors_palette_size ||
                        self.config.image_percent != self.config_window.old_image_percent/100.0
                || self.config_window.is_previewed
                    ;
                
                self.config.set_colors_count(self.config_window.old_colors_palette_size);
                self.config.set_size(self.config_window.old_image_percent/100.0);
                
                self.dithered_image.reload(ctx, &self.config);
                
                if (has_change) {
                    self.dithered_image.set_override_colors(self.config_window.palette_override.clone(), ctx);
                }
                self.config_window.close();
            }
            ConfigWindowCommands::Reload => {
                let (colors_count, size) = self.config_window.get_data_cfg();

                self.config.set_colors_count(colors_count);
                self.config.set_size(size);


                self.dithered_image.reload(ctx, &self.config);

                self.config_window.clear_last_command();
                self.palette_menu.clear_selection();
            }
        }
    }

    pub fn match_palette_panel_commands(&mut self, ctx: &Context) {

        match self.palette_menu.last_command {
            PaletteMenuCommand::None => {}
            PaletteMenuCommand::OpenPaletteWindow => {

                self.color_swap.open_window(self.palette_menu.selected.unwrap(), &self.dithered_image);

                self.palette_menu.clear_selection();
            }
        }
    }

    pub fn match_color_swap_commands(&mut self, ctx: &Context) {
        match self.color_swap.last_command {

            ColorReplaceCommand::None => {}
            ColorReplaceCommand::Replace => {
                if (!self.color_swap.previewed) {
                    self.dithered_image.replace_color(ctx, self.color_swap.end_color, self.color_swap.palette_id);
                }
                self.color_swap.close();
            }
            ColorReplaceCommand::Cancel => {
                if (self.color_swap.previewed) {
                    self.dithered_image.replace_color(ctx, self.color_swap.start_color, self.color_swap.palette_id);
                }
                self.color_swap.close();
            }
            ColorReplaceCommand::Preview => {
                self.dithered_image.replace_color(ctx, self.color_swap.end_color, self.color_swap.palette_id);
                self.color_swap.clear_command();
            }
            ColorReplaceCommand::Reset => {
                self.dithered_image.replace_color(ctx, self.dithered_image.get_pure_palette_colors()[self.color_swap.palette_id], self.color_swap.palette_id);
                self.color_swap.close();
            }
        }
    }
    fn open_save_file(&mut self, ctx: &egui::Context){

        if (!self.dithered_image.has_image()) {return};

        let (w, h) = self.dithered_image.size();
        let bytes: &[u8] = self.dithered_image.get_bytes();

        // проверка, чтобы не сохранить мусор
        if bytes.len() != w * h * 4 {
            eprintln!("Bad buffer size: {} != {}", bytes.len(), w * h * 4);
            return;
        }

        // 1) диалог сохранения
        let Some(mut path) = rfd::FileDialog::new()
            .set_file_name("dithered.png")
            .add_filter("PNG Image", &["png"])
            .save_file()
        else {
            return; // отмена
        };

        // 2) если юзер не написал расширение — добавим .png
        if path.extension().is_none() {
            path.set_extension("png");
        }

        // 3) сохраняем PNG через image
        // image::RgbaImage::from_raw забирает Vec<u8>, поэтому копируем bytes
        let buf = bytes.to_vec();

        let Some(img) = image::RgbaImage::from_raw(w as u32, h as u32, buf) else {
            eprintln!("Failed to create RgbaImage from raw bytes");
            return;
        };

        if let Err(e) = img.save_with_format(&path, image::ImageFormat::Png) {
            eprintln!("Save failed: {e}");
            // если хочешь — покажи свой OkDialog
            // self.ok_dialog.open_dialog("Ошибка", "OK", &format!("Не удалось сохранить:\n{e}"));
            return;
        }
    }
    fn open_image_picker(&mut self, ctx: &egui::Context) {
        let path: Option<PathBuf> = rfd::FileDialog::new()
            .add_filter("Image", &["png", "jpg", "jpeg", "bmp", "tga", "gif"])
            .pick_file();

        let Some(path) = path else { return; };

        match image::open(&path) {
            Ok(img) => {
                // тут делай что тебе надо: сохранить оригинал, пересчитать, обновить texture и т.д.
                self.dithered_image.upload_image(&path, ctx, &self.config, true);
            }
            Err(e) => {
                self.ok_dialog.open_dialog("Ошибка", "OK", &format!("Не удалось открыть файл:\n{e}"));
            }
        }
    }
}


impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {



        self.top_menu.update_menu(ctx, &self.dithered_image, !self.ok_dialog.is_open && !self.color_swap.is_open);
        self.dithered_image.update(ctx);
        self.palette_menu.update_palette(ctx, &self.dithered_image, !self.ok_dialog.is_open && !self.config_window.is_open && !self.color_swap.is_open);
        self.color_swap.update_color_window(ctx);

        self.ok_dialog.update(ctx);
        self.config_window.update(ctx);

        self.match_top_panel_commands(ctx);
        self.match_config_window_commands(ctx);
        self.match_palette_panel_commands(ctx);
        self.match_color_swap_commands(ctx);
        // self.open_file_dialog.open_dialog();
    }
}
