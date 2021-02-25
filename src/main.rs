use inputbot::{KeybdKey::*, MouseButton, *};
use std::{
    io::{stdout, Write},
    sync::Mutex,
    thread::sleep,
    time::Duration,
};

const SECONDS_BETWEEN_CLICKS: u64 = 6;
const SECOND: Duration = Duration::from_secs(1);

fn main() {
    println!(include_str!("startup_message.txt"));

    let state = Mutex::new(State::new());

    DeleteKey.bind(move || {
        {
            let mut state = state.lock().unwrap();
            if state.already_clicking {
                state.continue_clicking = false;
                print!("Stopping click loop...");
                stdout().flush().unwrap();
                return;
            } else {
                state.already_clicking = true;
                println!("Starting a click loop.");
            }
        }
        loop {
            MouseButton::LeftButton.press();
            MouseButton::LeftButton.release();

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
