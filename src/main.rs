use inputbot::{KeybdKey::*, MouseButton, *};
use std::{io::{Write, stdout}, sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    }, thread::sleep, time::Duration};

const SECONDS_BETWEEN_CLICKS: u64 = 6;
const SECOND: Duration = Duration::from_secs(1);

fn main() {
    println!("Starting up left_click_bot.");
    println!("Press Delete to start the click loop.");
    println!("Right click to toggle continue_clicking.");
    println!("Press Backspace to shut down.");
    let continue_clicking = Arc::new(AtomicBool::new(true));

    {
        let continue_clicking = continue_clicking.clone();
        let already_clicking = Arc::new(AtomicBool::new(false));

        DeleteKey.bind(move || {
            if already_clicking.load(Ordering::SeqCst) {
                println!("Can't start a click loop since one is already going.");
                return;
            } else {
                already_clicking.store(true, Ordering::SeqCst);
                println!("Starting a click loop.");
            }
            loop {
                for _ in 0..SECONDS_BETWEEN_CLICKS {
                    if !continue_clicking.load(Ordering::SeqCst) {
                        already_clicking.store(false, Ordering::SeqCst);
                        continue_clicking.store(true, Ordering::SeqCst);
                        println!("Stopped click loop.");
                        return;
                    }

                    sleep(SECOND);
                }

                MouseButton::LeftButton.click();
            }
        });
    }

    {
        let continue_clicking = continue_clicking.clone();
        MouseButton::RightButton.bind(move || {
            continue_clicking.fetch_xor(true, Ordering::SeqCst);
            print!("Stopping click loop...");
            stdout().flush().unwrap();
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
