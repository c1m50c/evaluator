use super::shell::{ShellError, shell_panic};
use super::parser::enums::Statement;
use std::process::exit;


/// Struct for evaluating and executing [`Statement`]s from a [`Parser`].
pub struct Evaluator {
    /// Contains [`Statement`]s usually obtained from a [`Parser`], to be looked through and evaluated.
    statements: Vec<Statement>,

    /// Current `index` within the `statements` field.
    position: usize,
}


impl Evaluator {
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