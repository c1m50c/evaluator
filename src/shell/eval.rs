use super::error::*;
use super::super::parser::ParsedData;
use super::super::parser::enums::*;
use std::process::exit;


pub fn evaluate_statements(data: ParsedData) {
    for s in data {
        evalulate(s);
    }
}


/// Evaluates a given statement, processing it and handling its contents.
#[allow(unreachable_patterns)]
fn evalulate(statement: Statement) {
    match statement {
        Statement::Arithmetic(x) => {
            match x {
                Expression::Arithmetic(a, operator, b) => {
                    let result;
                    
                    if let (Expression::Float(a), Expression::Float(b)) = (*a, *b) {
                        match operator {
                            ArithmeticOperation::Addition => result = a + b,
                            ArithmeticOperation::Subtraction => result = a - b,
                            ArithmeticOperation::Multiplication => result = a * b,
                            ArithmeticOperation::Division => result = a / b,
                            ArithmeticOperation::Modulo => result = a % b,
                            ArithmeticOperation::Pow => result = a.powf(b),

                            _ => shell_panic(
                                format!("Unimplemented Arithmetic Operation '{}'.", operator).as_ref(),
                                ShellError::EvaluationError
                            ),
                        }
                    } else {
                        shell_panic(
                            "Arithmetic Expression does not contain valid numbers in both Expressions.",
                            ShellError::EvaluationError
                        );
                    }

                    println!("{}", result);
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

                "help" | "h" => {
                    // TODO: Print helpful information
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