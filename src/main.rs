use inputbot::{KeybdKey::*, MouseButton, *};
use std::{
    io::{stdout, Write},
    sync::{
        Arc,
        Mutex
    },
    thread::sleep,
    time::Duration,
};

const SECONDS_BETWEEN_CLICKS: u64 = 3;
const SECOND: Duration = Duration::from_secs(1);

fn main() {
    println!(include_str!("startup_message.txt"));

    let state = Arc::new(Mutex::new(State::with_interval(SECONDS_BETWEEN_CLICKS)));

    {
        let state = state.clone();
        RControlKey.bind(move || {
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
                let seconds_between_clicks: u64;
                {
                    let mut state = state.lock().unwrap();
                    seconds_between_clicks = state.seconds_between_clicks;
                    if !state.continue_clicking {
                        state.already_clicking = false;
                        state.continue_clicking = true;
                        println!("Stopped click loop.");
                        return;
                    } else {
                        MouseButton::LeftButton.press();
                        MouseButton::LeftButton.release();
                    }
                }
                for _ in 0..seconds_between_clicks {
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

        InsertKey.bind(move || {
            let mut state = state.lock().unwrap();
            if state.seconds_between_clicks < u64::MAX {
                state.seconds_between_clicks += 1;
            }

            println!("Seconds between clicks: {}", state.seconds_between_clicks);
        });
    }

    {
        let state = state.clone();

        DeleteKey.bind(move || {
            let mut state = state.lock().unwrap();
            if state.seconds_between_clicks > u64::MIN {
                state.seconds_between_clicks -= 1;
            }

            println!("Seconds between clicks: {}", state.seconds_between_clicks);
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
    seconds_between_clicks: u64,
}

impl State {
    fn with_interval(interval: u64) -> Self {
        State {
            continue_clicking: true,
            already_clicking: false,
            seconds_between_clicks: interval,
        }
    }
}
