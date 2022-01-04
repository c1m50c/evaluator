use term_painter::{ToStyle, Color::*};
use std::panic::set_hook;
use core::fmt;


/// Enum representing errors for use in `shell_panic`.
#[derive(Debug, Clone, Copy)]
pub enum ShellError {
    EvaluationError,
    ParsingError,
    SyntaxError,
    UnknownError,
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


/// Similar to `expect()` in `Option` but throws a `shell_panic()` instead.
/// 
/// ## Example:
/// ```rust
/// let some: Option<bool> = Some(true);
/// let none: Option<bool> = None;
/// 
/// let some_true: bool = shell_expect_some(
///     some,
///     "This will be equal to `true`.",
///     ShellError::UnknownError
/// );
/// 
/// assert_eq!(some_true, true);
/// 
/// let this_will_panic: bool = shell_expect_some(
///     none,
///     "This will panic due to `none` being equal to `None`.",
///     ShellError::UnkownError
/// );
/// ```
pub fn shell_expect_some<T>(option: Option<T>, message: &str, error: ShellError) -> T {
    if option.is_none() { shell_panic(message, error); }
    return option.unwrap();
}