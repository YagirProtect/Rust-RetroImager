use crate::classes::c_config::Config;
use crate::classes::c_rgb16::Rgb16;
use crate::classes::t_widget::UIWidget;
use crate::image_utils::{build_palette_median_cut_rgba16, dither_fs_palette_rgba16_to_rgba8, pack_rgb, resize_interleaved_nearest, rgb16_to_u8, rgb16_to_u8_exact, rgba8_to_rgba16, set_texture};
use eframe::egui;
use std::path::PathBuf;
use std::collections::HashMap;


pub struct DitheredImage {
    tex: Option<egui::TextureHandle>,
    texDithered: Option<egui::TextureHandle>,
    texPure: Option<egui::TextureHandle>,
    w: usize,
    h: usize,
    is_loaded: bool,
    draw_dithered: bool,

    image_bytes8: Vec<u8>,
    image_bytes16: Vec<u16>,
    image_bytes8_dithered_pure: Vec<u8>,
    image_bytes8_dithered: Vec<u8>,

    palette : Vec<Rgb16>,
    palette_override : Vec<Rgb16>,

    last_path_buff: Option<PathBuf>,
}

impl Default for DitheredImage {
    fn default() -> Self {
        Self{
            tex: None,
            texDithered: None,
            texPure: None,
            w: 512,
            h: 512,
            is_loaded: false,
            image_bytes8: vec![],
            image_bytes16: vec![],
            image_bytes8_dithered: vec![],
            image_bytes8_dithered_pure: vec![],
            draw_dithered: false,
            palette: vec![],
            palette_override: vec![],
            last_path_buff: None,
        }
    }
}
impl UIWidget for DitheredImage{
    fn update(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {

            if (self.is_loaded) {
                let mut tex = self.tex.as_ref().unwrap();

                if (self.draw_dithered) {
                    tex = self.texDithered.as_ref().unwrap();
                }

                let rect = ui.available_rect_before_wrap();
                ui.allocate_rect(rect, egui::Sense::hover());

                // contain
                let img_rect = Self::contain_rect(rect, self.w as f32, self.h as f32);
                let uv = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0));
                ui.painter().image(tex.id(), img_rect, uv, egui::Color32::WHITE);
            }else{
                ui.centered_and_justified(|ui|{
                    ui.label("Empty\nFile>Open...");
                });
            }
        });
    }
}


impl DitheredImage {
    pub fn new(ctx: &egui::Context) -> Self {
        let img = DitheredImage::default();
        img
    }
    pub fn get_bytes(&self) -> &Vec<u8> {
        return &self.image_bytes8_dithered;
    }

    pub fn size(&self) -> (usize, usize) {
        (self.w, self.h)
    }

    pub fn upload_image(&mut self, path: &PathBuf, ctx: &egui::Context, config: &Config, from_open_file: bool) {
        let img = match image::open(path) {
            Ok(img) => img,
            Err(e) => {
                eprintln!("Failed to open image: {e}");
                return;
            }
        };

        // 1) Декодим ОДИН раз в RGBA8
        let rgba8_full = img.to_rgba8();
        let (w0, h0) = rgba8_full.dimensions();

        // 2) Pure texture (оригинал)
        set_texture(
            &mut self.texPure,
            ctx,
            "framebuffer_pure",
            w0 as usize,
            h0 as usize,
            rgba8_full.as_raw(),
        );

        // 3) Ресайзим ТОЛЬКО RGBA8
        // ВАЖНО: scale должен быть 0.5, 1.0 и т.д. (если у тебя проценты — дели на 100.0)
        let scale = config.image_percent; // например 0.5
        let (resized8, nw, nh) = resize_interleaved_nearest::<u8, 4>(
            rgba8_full.as_raw(),
            w0 as usize,
            h0 as usize,
            scale,
        );

        self.image_bytes8 = resized8;
        self.w = nw;
        self.h = nh;

        // 4) Дешево делаем RGBA16 уже на уменьшенном изображении
        rgba8_to_rgba16(&self.image_bytes8, &mut self.image_bytes16);

        // 5) Texture для отображения уменьшенного
        set_texture(
            &mut self.tex,
            ctx,
            "framebuffer_scaled",
            self.w,
            self.h,
            &self.image_bytes8,
        );

        self.last_path_buff = Some(path.clone());
        self.is_loaded = true;

        // 6) Дизеринг (самый тяжёлый этап)
        self.dither(ctx, config, from_open_file);
    }
    pub fn has_image(&self) -> bool {
        self.is_loaded
    }

    pub fn contain_rect(outer: egui::Rect, img_w: f32, img_h: f32) -> egui::Rect {
        let ow = outer.width().max(1.0);
        let oh = outer.height().max(1.0);
        let s = (ow / img_w.max(1.0)).min(oh / img_h.max(1.0));
        let size = egui::vec2(img_w * s, img_h * s);
        let min = outer.center() - size * 0.5;
        egui::Rect::from_min_size(min, size)
    }


    pub fn reload(&mut self, ctx: &egui::Context, config: &Config) {
        if (self.has_image()){
            if (!self.last_path_buff.is_none()){
                let buff = self.last_path_buff.clone().unwrap();
                self.upload_image(&buff, ctx, config, false);
            }
        }
    }
    pub fn dither(&mut self, ctx: &egui::Context, config: &Config, from_open_file: bool) {
        if !self.is_loaded { return; }

        let k = (config.colors_palette_size as usize).max(2);

        if from_open_file || self.palette.len() != k {
            self.palette = build_palette_median_cut_rgba16(&self.image_bytes16, self.w, self.h, k);
            self.palette_override = self.palette.clone();
        }

        self.image_bytes8_dithered.resize(self.w * self.h * 4, 0);

        dither_fs_palette_rgba16_to_rgba8(
            &self.image_bytes16,
            &mut self.image_bytes8_dithered,
            self.w,
            self.h,
            &self.palette,
        );

        self.image_bytes8_dithered_pure.clear();
        self.image_bytes8_dithered_pure.extend_from_slice(&self.image_bytes8_dithered);

        let img = egui::ColorImage::from_rgba_unmultiplied([self.w, self.h], &self.image_bytes8_dithered);
        match &mut self.texDithered {
            Some(t) => t.set(img, egui::TextureOptions::NEAREST),
            None => self.texDithered = Some(ctx.load_texture("framebuffer_dithered", img, egui::TextureOptions::NEAREST)),
        }

        self.draw_dithered = true;
    }

    pub fn set_override_colors(&mut self, p: Vec<Rgb16>, ctx: &egui::Context) {
        for (i, &c) in p.iter().enumerate() {
            self.replace_color(ctx, c, i);
        }
    }


    pub fn replace_color(&mut self, ctx: &egui::Context, new_color: Rgb16, palette_id: usize) {
        if palette_id >= self.palette.len() {
            return;
        }
        if self.palette_override.len() != self.palette.len() {
            self.palette_override = self.palette.clone();
        }

        self.palette_override[palette_id] = new_color;

        // пересобрать dithered из pure + override
        self.apply_palette_override_to_dithered(ctx);
    }
    pub fn get_palette_colors(&self) -> &Vec<Rgb16> {
        return &self.palette_override;
    }
    pub fn get_pure_palette_colors(&self) -> &Vec<Rgb16> {
        return &self.palette;
    }

    fn apply_palette_override_to_dithered(&mut self, ctx: &egui::Context) {
        // защиты
        if self.image_bytes8_dithered_pure.len() != self.w * self.h * 4 {
            return;
        }
        if self.palette.is_empty() {
            return;
        }
        if self.palette_override.len() != self.palette.len() {
            // если ещё не инициализирована — делаем дефолт "как палитра"
            self.palette_override = self.palette.clone();
        }

        // LUT: базовый цвет (RGB u8) -> id палитры
        let mut lut: HashMap<u32, usize> = HashMap::with_capacity(self.palette.len() * 2);
        for (i, &c16) in self.palette.iter().enumerate() {
            let (r, g, b) = rgb16_to_u8_exact(c16);
            lut.insert(pack_rgb(r, g, b), i);
        }

        // подготовим dst
        self.image_bytes8_dithered.resize(self.w * self.h * 4, 0);

        let src = &self.image_bytes8_dithered_pure;
        let dst = &mut self.image_bytes8_dithered;

        for p in (0..src.len()).step_by(4) {
            let pr = src[p];
            let pg = src[p + 1];
            let pb = src[p + 2];
            let pa = src[p + 3];

            if let Some(&id) = lut.get(&pack_rgb(pr, pg, pb)) {
                let (nr, ng, nb) = rgb16_to_u8_exact(self.palette_override[id]);
                dst[p] = nr;
                dst[p + 1] = ng;
                dst[p + 2] = nb;
                dst[p + 3] = pa; // альфу оставляем как в pure
            } else {
                // На всякий случай: если цвет не найден — копируем как есть
                dst[p] = pr;
                dst[p + 1] = pg;
                dst[p + 2] = pb;
                dst[p + 3] = pa;
            }
        }

        // обновляем текстуру
        let img = egui::ColorImage::from_rgba_unmultiplied([self.w, self.h], &self.image_bytes8_dithered);
        match &mut self.texDithered {
            Some(tex) => tex.set(img, egui::TextureOptions::NEAREST),
            None => {
                self.texDithered = Some(ctx.load_texture(
                    "framebuffer_dithered",
                    img,
                    egui::TextureOptions::NEAREST,
                ));
            }
        }
    }
}