#[allow(unused_imports)]
use term_painter::{ToStyle, Color::*};
use std::thread;


fn main() {
    loop {
        let execution_thread = thread::spawn(move || {
            if cfg!(debug_assertions) {}
        });

        let _ = execution_thread.join();
    }
}