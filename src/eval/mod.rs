use super::shell::{ShellError, shell_panic};
use super::parser::enums::Statement;
use std::process::exit;


#[allow(unreachable_patterns)]
fn evaluate_statement(statement: Statement) {
    match statement {
        Statement::Command(c) => match c.to_lowercase().as_str() {
            "quit" | "exit" => exit(0),

            c => shell_panic(
                ShellError::EvaluationError,
                format!("Statement::Command({:?}) cannot be evaluated, command '{}' does not exist.", c, c)
            ),
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