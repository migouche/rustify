use std::{
    sync::{atomic::{AtomicBool, AtomicI32, Ordering::SeqCst}, Arc},
    thread,
};


use eframe::egui;
use event_listener::Event;

use crate::music_player::{audio_control, AudioInfo};
pub struct Rustify {
    audio_info: Arc<AudioInfo>
}

impl Rustify {
    pub fn new() -> Self {

        // i should do an "atomic struct" for this stuff

        let audio = Arc::new(AudioInfo::new());

        /*let event = Arc::new(Event::new());
        let flag = Arc::new(AtomicBool::new(false));
        let progress = Arc::new(AtomicI32::new(0));*/
        thread::spawn(
            /*let event = event.clone();
            let flag = flag.clone();
            let progress = progress.clone();*/
            {
                let audio = audio.clone();
            
                move || audio_control(&audio)
            }
            
        );
        Rustify {
            audio_info: audio
        }
    }
}

impl eframe::App for Rustify {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rustify");
        });


        egui::TopBottomPanel::bottom(egui::Id::new("music control")).show(ctx, |ui| {
            // add music control here(centered)

            ui.horizontal_centered(|ui| {
                if ui.button("shuffle").clicked() {};
                if ui.button("prev").clicked() {};
                if ui.button("Play").clicked() {
                    println!("Play");
                    self.audio_info.toggle_pause(SeqCst)
                }
                if ui.button("next").clicked() {};
                if ui.button("loop").clicked() {};
            });
        });

        // music slider
        egui::TopBottomPanel::bottom(egui::Id::new("music slider")).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                let mut val = self.audio_info.load_progress(SeqCst);
                let val_cp = val;
                let r = ui.add(egui::Slider::new(&mut val, 0..=100).
                text(format!("{}:{}", val_cp / 60, val_cp % 60).as_str()));
                self.audio_info.store_progress(val, SeqCst);
                if r.changed() {
                    println!("slider changed to {}", val);
                }
                r
            }); 
        });
    }
}
