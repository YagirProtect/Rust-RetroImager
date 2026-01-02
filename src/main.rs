mod classes;
mod image_utils;
mod file_system_utils;

use std::sync::Arc;
use crate::classes::c_app::App;

fn load_icon() -> Arc<egui::IconData> {
    let bytes = include_bytes!("../assets/icon.png");

    let image = image::load_from_memory(bytes)
        .expect("icon decode failed")
        .to_rgba8();

    let (w, h) = image.dimensions();
    Arc::new(egui::IconData {
        rgba: image.into_raw(),
        width: w,
        height: h,
    })
}


fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_icon(load_icon()),
        ..Default::default()
    };
    eframe::run_native("Dithering Slave", native_options, Box::new(|cc| Ok(Box::new(App::new(cc)))));
}

