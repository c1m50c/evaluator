pub(crate) mod error;
pub(crate) mod eval;

use term_painter::{ToStyle, Color::*};
use std::io::{self, Write};


/// Returns the input from the Standard Input as a trimmed `String`.
/// ## Panics:
/// - If `io::stdin().read_line()` Fails.
pub fn input() -> String {
    let mut input: String = String::new();

    /* Prompt */
    print!("{}{}",
        Magenta.bold().paint("evalulator"),
        Yellow.bold().paint(":$ "),
    );

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line from standard input.");
    return input.trim().to_string();
}