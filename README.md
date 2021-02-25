# left_click_bot
A simple program that makes a left click every 6 seconds, though the duration between clicks can be easily adjusted in the code. Once the program is running, press the Delete key to start the clicking. In order to stop the clicking, click the right mouse button and wait until the beginning of the next click cycle. To start the clicking, just press the Delete key. To shutdown the program, press the Backspace key.

## Why does this exist?
Simple. I wanted to do something in a game while afk. The think I needed to do involved periodic left clicking. I had an idea, and now this exists.

## Compatibility
This has a dependency on the `inputbot` crate and should work on any platform supported by `inputbot`. At the time of writing this, the crates.io page for `inputbot` says that it supports Windows and Linux.