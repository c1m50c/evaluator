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


/// Slightly tweaked version of the `println!` Macro but as a Function.
/// If the build was built in `debug` mode this will print a formatted string to the standard output.
/// 
/// ## Example:
/// ```rust
/// let t = Token::Plus;
/// print_debug("Lexical Analysis", format!("We got some Token '{}'.", t).as_ref());
/// // "[ DEBUG ] Lexical Analysis: We got some Token 'Plus'." will be printed to the standard output.
/// ```
pub fn print_debug(title: &str, message: &str) {
    if cfg!(debug_assertions) {
        println!("{} {}: {}",
            Green.bold().paint("[ DEBUG ]"),
            Cyan.paint(title),
            message
        );
    }
}