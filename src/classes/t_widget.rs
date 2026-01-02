use eframe::egui;

pub trait UIWidget {
    fn update(&mut self, ctx: &egui::Context) {
        
    }
}