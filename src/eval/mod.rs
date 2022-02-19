#[allow(unused_imports)]
use term_painter::{ToStyle, Color::*};

use super::shell::{ShellError, shell_panic, output};
use super::parser::enums::Statement;
use super::lexer::token::Token;
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


fn evaluate_arithmetic(a: Statement, operator: Token, b: Statement) -> Result<f64, String> {
    let a = match a {
        Statement::Arithmetic(a, operator, b) => {
            evaluate_arithmetic(*a, operator, *b).unwrap_or_else(|err| {
                shell_panic(
                    ShellError::EvaluationError, err
                )
            })
        },

        Statement::Number(n) => n,

        _ => return Err(
            format!("Cannot evaluate Statement::{:?} into a floating-point number.", a)
        ),
    };

    let b = match b {
        Statement::Arithmetic(a, operator, b) => {
            evaluate_arithmetic(*a, operator, *b).unwrap_or_else(|err| {
                shell_panic(
                    ShellError::EvaluationError, err
                )
            })
        },
        
        Statement::Number(n) => n,

        _ => return Err(
            format!("Cannot evaluate Statement::{:?} into a floating-point number.", b)
        ),
    };

    let result = match operator {
        Token::Plus => a + b,
        Token::Minus => a - b,
        Token::Star => a * b,
        Token::ForwardSlash => a / b,
        Token::Caret => a.powf(b),

        _ => return Err(
            format!("Token::{:?} is not a valid Arithmetic Operator.", operator)
        ),
    };
    
    return Ok(result);
}


#[allow(unreachable_code)]
fn evaluate_mathematical_function(func: &str, number: f64) -> Result<(), String> {
    match func {
        "sqrt" => output(number.sqrt()),
        "sin" => output(number.sin()),
        "cos" => output(number.cos()),

        f => return Err(
            format!("Mathematical Function '{}' does not exist.", f)
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
                if let Err(err) = evaluate_command(c.to_lowercase().as_str()) {
                    shell_panic(
                        ShellError::EvaluationError,
                        err
                    );
                }
            },

            Statement::Arithmetic(a, operator, b) => {
                match evaluate_arithmetic(*a, operator, *b) {
                    Ok(n) => output(n),

                    Err(err) => shell_panic(
                        ShellError::EvaluationError, err
                    ),
                }
            },

            Statement::MathematicalFunction(s, n) => {
                if let Statement::Number(n) = *n {
                    if let Err(err) = evaluate_mathematical_function(s.to_lowercase().as_str(), n) {
                        shell_panic(
                            ShellError::EvaluationError, err
                        );
                    }
                }
            }
        
            _ => shell_panic(
                ShellError::EvaluationError,
                format!("Statement::{:?} cannot be evaluated.", s)
            ),
        }
    }
}