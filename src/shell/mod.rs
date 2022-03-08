pub mod err;

use term_painter::{ToStyle, Color::*};
use std::io::{self, Write};

pub use err::*;


/// Returns input from the standard input as a trimmed [`String`].
/// 
/// # Panics
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


/// Outputs a value to the standard output, formatting it with a prefixed `>>>`.
/// 
/// # Example
/// ```rust
/// shell::output(5.34); // Will print out ">>> 5.34".
/// ```
#[inline]
pub fn output<T: core::fmt::Display>(out: T) {
    println!("{}{}",
        Yellow.bold().paint(">>> "), out
    );
}


/// Prints the `category` and `info` with special debug formatting.
/// 
/// # Example
/// ```rust
/// // Will output "[ DEBUG ] Lexical Token Token::Plus" if built with the debug flag.
/// shell::debug_print("Lexical Token", Token::Plus);
/// ```
#[inline]
pub fn debug_print<C: core::fmt::Display, I: core::fmt::Debug>(category: C, info: I) {
    if cfg!(debug_assertions) {
        println!("{}{}{:?}",
            Green.bold().paint("[ DEBUG ] "),
            Cyan.bold().paint(category),
            info,
        );
    }
}