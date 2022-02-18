use super::shell::{ShellError, shell_panic};
use super::parser::enums::Statement;
use core::result::Result;
use std::process::exit;


/// Evaluates and executes a command [`Statement`],
/// returning a [`Result`] containing the error message if something were to go wrong.
#[allow(unreachable_code)]
fn evaluate_command(command: &str) -> Result<(), String> {
    match command {
        "quit" | "exit" => exit(0),

        c => return Err(
            format!("Statement::Command({:?}) cannot be evaluated, command does not exist.", c)
        ),
    }
    
    return Ok(());
}


#[inline]
#[allow(unreachable_patterns)]
pub fn evaluate(statements: Vec<Statement>) {
    for s in statements {
        match s {
            Statement::Command(c) => {
                if let Err(error_message) = evaluate_command(c.to_lowercase().as_str()) {
                    shell_panic(
                        ShellError::EvaluationError,
                        error_message
                    );
                }
            },
        
            _ => shell_panic(
                ShellError::EvaluationError,
                format!("Statement::{:?} cannot be evaluated.", s)
            ),
        }
    }
}