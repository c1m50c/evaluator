use term_painter::{ToStyle, Color::*};
use super::shell::{ShellError, shell_panic};
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

            Statement::Arithmetic(a, operator, b) => {
                match evaluate_arithmetic(*a, operator, *b) {
                    Ok(n) => println!("{}{}",
                        Yellow.bold().paint(">>> "), n
                    ),

                    Err(e) => shell_panic(
                        ShellError::EvaluationError, e
                    ),
                }
            },
        
            _ => shell_panic(
                ShellError::EvaluationError,
                format!("Statement::{:?} cannot be evaluated.", s)
            ),
        }
    }
}