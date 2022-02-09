pub mod err;

use term_painter::{ToStyle, Color::*};
use std::io::{self, Write};

pub use err::*;


/// Returns input from the standard input as a trimmed [`String`].
/// 
/// ## Panics
/// - If `io::stdin().read_line()` Fails.
#[inline]
pub fn input() -> String {
    let mut input = String::new();

    print!("{}{}",
        Magenta.bold().paint("evaluator"),
        Yellow.bold().paint(":$ "),
    );

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line from standard input.");

    return input.trim().to_string();
}


/// Prints the `category` and `info` with special debug formatting, does not ensure application is in debug mode.
#[inline]
pub fn debug_print(category: &str, info: &str) {
    println!("{}{}{}",
        Green.bold().paint("[ DEBUG ] "),
        Cyan.bold().paint(format!("{} ", category).as_str()),
        info,
    );
}