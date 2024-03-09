use rodio::{Decoder, OutputStream, Source, Sink};
use std::fs::File;
use std::io::BufReader;

use eframe::egui::{self, Button};


//

fn main() -> Result<(), eframe::Error> {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open("data/test.mp3").unwrap();
    //sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native("Rustify", options, Box::new(|_| Box::new(Rustify::new())))
}

struct Rustify {
    sink: Sink,
    stream: OutputStream,
    handle: rodio::OutputStreamHandle,
}

impl Rustify {
    fn new() -> Self
    {
        println!("Hello, world!");
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();
    
        let file = std::fs::File::open("data/test.mp3").unwrap();
        sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    
        sink.sleep_until_end();
        Rustify {
            sink
        }
    }
}



impl eframe::App for Rustify {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.sink.empty() {
            self.sink.append(Decoder::new(BufReader::new(File::open("data/test.mp3").unwrap())).unwrap());
            self.sink.play();
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("text");
            if ui.button("text").clicked(){
                if self.sink.is_paused() {
                    self.sink.play();
                } else {
                    self.sink.pause();
                }
                
            }

        });

    }
}
