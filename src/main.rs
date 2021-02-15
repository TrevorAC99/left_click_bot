use inputbot::{KeybdKey::*, MouseButton, *};
use std::{
    io::{stdout, Write},
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

const SECONDS_BETWEEN_CLICKS: u64 = 6;
const SECOND: Duration = Duration::from_secs(1);

fn main() {
    println!("Starting up left_click_bot.");
    println!("Press Delete to start the click loop.");
    println!("Right click to toggle continue_clicking.");
    println!("Press Backspace to shut down.");

    let state = Arc::new(Mutex::new(State::new()));

    {
        let state = state.clone();

        DeleteKey.bind(move || {
            {
                let mut state = state.lock().unwrap();
                if state.already_clicking {
                    println!("Can't start a click loop since one is already going.");
                    return;
                } else {
                    state.already_clicking = true;
                    println!("Starting a click loop.");
                }
            }
            loop {
                MouseButton::LeftButton.click();

                for _ in 0..SECONDS_BETWEEN_CLICKS {
                    {
                        let mut state = state.lock().unwrap();
                        if !state.continue_clicking {
                            state.already_clicking = false;
                            state.continue_clicking = true;
                            println!("Stopped click loop.");
                            return;
                        }
                    }

                    sleep(SECOND);
                }
            }
        });
    }

    {
        let state = state.clone();
        MouseButton::RightButton.bind(move || {
            let mut state = state.lock().unwrap();
            if state.already_clicking {
                state.continue_clicking = false;
                print!("Stopping click loop...");
                stdout().flush().unwrap();
            }
        });
    }

    BackspaceKey.bind(|| {
        println!("Shutting down");
        std::process::exit(0);
    });

    handle_input_events();
}

struct State {
    continue_clicking: bool,
    already_clicking: bool,
}

impl State {
    fn new() -> Self {
        State {
            continue_clicking: true,
            already_clicking: false,
        }
    }
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
