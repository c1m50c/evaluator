use super::parser::enums::{Statement, BinaryOperation};
use super::shell::{ShellError, shell_panic, output};
use std::process::exit;


/// Struct for evaluating and executing [`Statement`]s from a [`Parser`].
pub struct Evaluator {
    /// Contains [`Statement`]s usually obtained from a [`Parser`], to be looked through and evaluated.
    statements: Vec<Statement>,

    /// Current `index` within the `statements` field.
    position: usize,
}


impl Evaluator {
    /// Attempts to matches an `Arithmetic` or `Number` [`Statement`] into a [`f64`], helper function for [`evaluate_arithmetic`].
    // TODO: Maybe make this an inner function for `evaluate_arithmetic`.
    #[inline]
    fn match_arithmetic_statement(statement: Statement) -> Result<f64, String> {
        return match statement {
            Statement::Arithmetic(a, o, b) => {
                let eval = Self::evaluate_arithmetic(*a, o, *b);
    
                if let Err(err) = eval { Err(err) }
                else { Ok(eval.unwrap()) }
            },
    
            Statement::Number(n) => Ok(n),
    
            s => return Err(
                format!("Statement::{:?} is not a Number or Arithmetic Statement.", s)
            )
        };
    }
    
    /// Attemps to evaluate an `Arithmetic` [`Statement`] into a [`f64`] number.
    // TODO: Cleanup this Code.
    #[allow(unreachable_patterns)]
    fn evaluate_arithmetic(a: Statement, o: BinaryOperation, b: Statement) -> Result<f64, String> {
        let a = Self::match_arithmetic_statement(a);
        let b = Self::match_arithmetic_statement(b);

        if let Err(err) = a { return Err(err); }
        if let Err(err) = b { return Err(err); }
        let a = a.unwrap(); let b = b.unwrap();

        return match o {
            BinaryOperation::Addition => Ok(a + b),
            BinaryOperation::Subtraction => Ok(a - b),
            BinaryOperation::Multiplication => Ok(a * b),
            BinaryOperation::Division => Ok(a / b),
            BinaryOperation::Power => Ok(a.powf(b)),

            o => Err(
                format!("BinaryOperation::{:?} is non-arithmetic.", o)
            ),
        };
    }

    /// Evaluates the [`String`] from a `Command` [`Statement`].
    #[allow(unreachable_code)]
    fn evaluate_command(&self, cmd: String) -> Result<(), String> {
        match cmd.as_str() {
            "quit" | "exit" => exit(0),

            c => return Err(
                format!("Cannot evaluate Statement::Command({:?}), command does not exist.", c)
            ),
        }

        return Ok(());
    }
}


impl Evaluator {
    /// Constructs a new [`Evaluator`] out of a [`Vec`] of [`Statement`]s, usually obtained from a [`Parser`].
    #[inline]
    pub const fn new(statements: Vec<Statement>) -> Self {
        return Self {
            statements,
            position: 0,
        };
    }

    /// Returns a reference to the [`Statement`] at the current `position` in the [`Evaluator`]'s `statements` field.
    #[inline]
    pub fn current(&self) -> Option<&Statement> {
        if self.statements.len() <= self.position { return None; }
        return Some(&self.statements[self.position]);
    }

    /// Evaluates and executes the [`Statement`]s within the [`Evaluator`].
    #[allow(unreachable_patterns)]
    pub fn evaluate(&mut self) {
        while let Some(statement) = self.current() {
            match statement.clone() {
                Statement::Arithmetic(a, o, b) => {
                    match Self::evaluate_arithmetic(*a, o, *b) {
                        Ok(n) => output(n),
                        Err(err) => shell_panic(
                            ShellError::EvaluationError, err
                        ),
                    }
                },

                Statement::Command(cmd) => {
                    if let Err(err) = self.evaluate_command(cmd.to_lowercase()) {
                        shell_panic(
                            ShellError::EvaluationError, err
                        );
                    }
                },
                
                s => shell_panic(
                    ShellError::EvaluationError,
                    format!("Cannot evaluate Statement::{:?}.", s)
                ),
            }

            self.position += 1;
        }
    }
}