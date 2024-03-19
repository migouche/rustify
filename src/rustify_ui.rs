use std::{
    sync::{atomic::{AtomicBool, AtomicI32}, Arc},
    thread,
};

use eframe::egui;
use event_listener::Event;

use crate::music_player::audio_control;
pub struct Rustify {
    event: Arc<Event>,
    flag: Arc<AtomicBool>,
    progress: Arc<AtomicI32>,
}

impl Rustify {
    pub fn new() -> Self {
        let event = Arc::new(Event::new());
        let flag = Arc::new(AtomicBool::new(false));
        let progress = Arc::new(AtomicI32::new(0));
        thread::spawn({
            let event = event.clone();
            let flag = flag.clone();
            let progress = progress.clone();
            move || {
                audio_control(event, flag, progress);
            }
        });
        Rustify {
            event: event.clone(),
            flag: flag.clone(),
            progress: progress.clone(),
        }
    }
}

impl eframe::App for Rustify {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("text");
            if ui.button("text").clicked() {
                println!("text");
                self.flag.store(true, std::sync::atomic::Ordering::SeqCst);
                assert_eq!(self.event.notify(1), 1);
            }
        });


        egui::TopBottomPanel::bottom(egui::Id::new("music control")).show(ctx, |ui| {
            // add music control here(centered)

            ui.horizontal_centered(|ui| {
                if ui.button("shuffle").clicked() {};
                if ui.button("prev").clicked() {};
                if ui.button("Play").clicked() {
                    println!("Play");
                    self.flag.store(true, std::sync::atomic::Ordering::SeqCst);
                    assert_eq!(self.event.notify(1), 1);
                }
                if ui.button("next").clicked() {};
                if ui.button("loop").clicked() {};
            });
        });

        // music slider
        egui::TopBottomPanel::bottom(egui::Id::new("music slider")).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                let mut val = self.progress.load(std::sync::atomic::Ordering::SeqCst);
                let val_cp = val;
                let r = ui.add(egui::Slider::new(&mut val, 0..=100).
                text(format!("{}:{}", val_cp / 60, val_cp % 60).as_str()));
                self.progress.store(val, std::sync::atomic::Ordering::SeqCst);
                if r.changed() {
                    println!("slider changed to {}", val);
                }
                r
            }); 
        });
    }
}
