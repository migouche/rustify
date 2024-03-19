use std::sync::{atomic::{AtomicBool, AtomicI32}, Arc};

use event_listener::{Event, Listener};

pub fn audio_control(event: Arc<Event>, flag: Arc<AtomicBool>, progress: Arc<AtomicI32>) -> !{
    let mut p;

    loop {
        let listener = event.listen();
        p = progress.load(std::sync::atomic::Ordering::SeqCst);



        if flag.load(std::sync::atomic::Ordering::SeqCst) {
            println!("audio");
            flag.store(false, std::sync::atomic::Ordering::SeqCst);
            p += 1;
            progress.store(p, std::sync::atomic::Ordering::SeqCst);
        }

        listener.wait();
    }
}
