use std::sync::{atomic::{AtomicBool, AtomicI32, Ordering::{self, SeqCst}}, Arc};

use event_listener::{Event, Listener};

pub fn audio_control(audio: &AudioInfo) -> !{
    let mut p;

    loop {
        let listener = audio.listen();
        p = audio.progress.load(SeqCst);



        if audio.paused(SeqCst) {
            println!("audio");
            p += 1;
            audio.progress.store(p, SeqCst);
        }

        listener.wait();
    }
}

pub struct AudioInfo
{
    event: Event,
    progress: AtomicI32,
    pause: AtomicBool,

}

impl AudioInfo {
    pub fn new() -> Self
    {
        AudioInfo{
            event: Event::new(),
            progress: AtomicI32::new(0),
            pause: AtomicBool::new(false)
        }
    }

    pub fn listen(&self) -> event_listener::EventListener
    {
        self.event.listen()
    }

    pub fn store_progress(&self, val: i32, ordering: Ordering) -> ()
    {
        self.progress.store(val, ordering);
        self.notify();
    }

    pub fn load_progress(&self, ordering: Ordering) -> i32
    {
        self.progress.load(ordering)
    }

    pub fn toggle_pause(&self, ordering: Ordering) -> ()
    {
        self.pause.store(!self.paused(ordering), ordering);
        self.notify()
    }

    pub fn paused(&self, ordering: Ordering) -> bool
    {
        self.pause.load(ordering)
    }

    pub fn notify(&self) -> ()
    {
        assert_eq!(self.event.notify(1), 1)
    }
}