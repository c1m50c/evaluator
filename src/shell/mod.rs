use term_painter::{ToStyle, Color::*};
use std::io::{self, Write};


/// Returns the Input from the Standard Input as a String.
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