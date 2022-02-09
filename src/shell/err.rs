use term_painter::{ToStyle, Color::*};
use core::fmt::{Display, Debug};
use std::panic::set_hook;


/// For representing certain errors in [`shell_panic`].
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ShellError {
    /// Generic error thrown when you don't know which else to use.
    UnknownError,
    
    /// Normally represents a error ocurring during lexical analyisis.
    SyntaxError,

    /// Errors that occur during parsing.
    ParsingError,
}


/// Panics the execution thread of the shell, printing the `error` and the `msg` to the standard output.
/// Similar to the `panic!()` macro, but without the long traceback message associated with it.
#[inline]
pub fn shell_panic<E: Debug, S: Display>(error: E, msg: S) -> ! {
    println!("{}{}",
        Red.bold().paint(format!("{:?}: ", error)),
        format!("{}", msg)
    );
    
    set_hook(Box::new(|_| {  }));
    panic!();
}