use term_painter::{ToStyle, Color::*};
use std::panic::set_hook;
use core::fmt;


#[derive(Debug, Clone, Copy)]
pub enum ShellError {
    EvaluationError,
    ParsingError,
    SyntaxError,
}


impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{:?}", self);
    }
}


/// Panics the execution thread of the shell, printing the `error` and the `message` to the standard output.
/// Similar to the `panic!` macro, but without the long print message associated with it.
/// 
/// ## Example:
/// ```rust
/// shell_panic("Something went wrong in lexical analysis.", ShellError::SyntaxError);
/// // "SyntaxError: Something went wrong in lexical analysis." will be printed.
/// ```
pub fn shell_panic(message: &str, error: ShellError) -> ! {
    println!("{}{}",
        Red.bold().paint(format!("{}: ", error)),
        BrightWhite.paint(format!("{}", message))
    );

    set_hook(Box::new( |_| {  } )); // To prevent `panic!` from printing anything.
    panic!();
}