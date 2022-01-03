use core::fmt;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellError {
    SyntaxError,
    EvaluationError,
}


impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{:?}", self);
    }
}


/// Panics the execution thread of the shell, printing the `error` and the `message` to the standard output.
/// Similar to the `panic!` macro, but without the backtrace.
/// 
/// ## Example:
/// ```rust
/// shell_panic("Something went wrong in lexical analysis.", ShellError::SyntaxError);
/// // "SyntaxError: Something went wrong in lexical analysis." will be printed.
/// ```
pub fn shell_panic(message: &str, error: ShellError) -> ! {
    println!("{}: {}", error, message);
    /*
        FIXME:TODO:
        Kill Thread on call, currently goes into a infinite loop without exiting.
    */
    loop {  }
}