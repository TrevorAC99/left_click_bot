use inputbot::{KeybdKey::*, MouseButton, *};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread::sleep,
    time::Duration,
};

const SECONDS_BETWEEN_CLICKS: u64 = 6;
const DURATION_BETWEEN_CLICKS: Duration = Duration::from_secs(SECONDS_BETWEEN_CLICKS);

fn main() {
    println!("Starting up left_click_bot.\nPress Delete to start the click loop.\nRight click to toggle continue_clicking.\nPress Backspace to shut down.");
    let continue_clicking = std::sync::Arc::new(AtomicBool::new(true));

    {
        let continue_clicking = continue_clicking.clone();
        let already_clicking = std::sync::Arc::new(AtomicBool::new(false));
        DeleteKey.bind(move || {
            if already_clicking.load(Ordering::SeqCst) {
                println!("Can't start a click loop since one is already going.");
                return;
            } else {
                already_clicking.store(true, Ordering::SeqCst);
                println!("Starting a click loop.");
            }
            loop {
                if !continue_clicking.load(Ordering::SeqCst) {
                    already_clicking.store(false, Ordering::SeqCst);
                    continue_clicking.store(true, Ordering::SeqCst);
                    println!("Stopped click loop.");
                    break;
                }

                MouseButton::LeftButton.click();

                sleep(DURATION_BETWEEN_CLICKS);
            }
        });
    }

    {
        let continue_clicking = continue_clicking.clone();
        MouseButton::RightButton.bind(move || {
            let prev = continue_clicking.fetch_xor(true, Ordering::SeqCst);
            println!("continue_clicking changed from {} to {}", prev, !prev);
        });
    }

    BackspaceKey.bind(|| {
        println!("Shutting down");
        std::process::exit(0);
    });

    handle_input_events();
}

trait Clickable {
    fn click(self);
}

impl Clickable for MouseButton {
    fn click(self) {
        self.clone().press();
        self.release();
    }
}
