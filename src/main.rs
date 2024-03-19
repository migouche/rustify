use eframe::egui;

mod music_player;
mod rustify_ui;
//

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Rustify",
        options,
        Box::new(|_| Box::new(rustify_ui::Rustify::new())),
    )
}
