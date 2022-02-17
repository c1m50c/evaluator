use super::shell::{ShellError, shell_panic};
use super::parser::enums::Statement;
use core::result::Result;
use std::process::exit;


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


#[allow(unreachable_patterns)]
fn evaluate_statement(statement: Statement) {
    match statement {
        Statement::Command(c) => {
            if let Err(error_message) = evaluate_command(c.to_lowercase().as_str()) {
                shell_panic(
                    ShellError::EvaluationError,
                    error_message
                );
            }
        },
    
        s => shell_panic(
            ShellError::EvaluationError,
            format!("Statement::{:?} cannot be evaluated.", s)
        ),
    }
}


#[inline]
pub fn evaluate(statements: Vec<Statement>) {
    for s in statements {
        evaluate_statement(s);
    }
}