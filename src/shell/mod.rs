pub(crate) mod error;

use super::parser::enums::{Statement, Expression};
use term_painter::{ToStyle, Color::*};
use std::io::{self, Write};
use std::process::exit;
use error::*;


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


#[allow(unreachable_patterns)]
pub fn evalulate(statement: Statement) {
    match statement {
        Statement::Arithmetic(x) => {
            match x {
                Expression::Arithmetic(a, operator, b) => {

                },

                _ => shell_panic(
                    "Arithmetic Statement does not contain an Arithmetic Expression.",
                    ShellError::EvaluationError
                ),
            }
        },

        Statement::Command(x) => {
            match x.to_lowercase().as_str() {
                "quit" | "exit" => {
                    exit(0);
                },

                _ => shell_panic(
                    format!("Unimplemented Command '{}'.", x).as_ref(),
                    ShellError::EvaluationError
                ),
            }
        },

        _ => shell_panic(
            "Unimplemented Statement.",
            ShellError::EvaluationError
        ),
    }
}