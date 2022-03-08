pub mod enums;

use super::shell::{ShellError, shell_panic};
use super::lexer::{token::Token, Lexer};
use core::convert::TryInto;
use std::vec::Vec;
use enums::*;


/// Creates [`Statement`]s that describe operations and expressions for evaluation.
pub struct Parser {
    /// [`Vec`] of [`Statement`]s to be constructed after the `parse()` method is called.
    statements: Vec<Statement>,
    
    /// [`Lexer`] of the [`Parser`], used in constructing the [`Statement`]s of the [`Parser`].
    lexer: Lexer,
}


impl Parser {
    /// Attempts to parse a `number` [`String`] to a 64-bit floating-point number.
    ///
    /// # Panics
    /// - If parsing the [`String`] into a [`f64`] fails.
    #[inline]
    fn parse_number(number: String) -> Statement {
        let result = number.parse::<f64>()
            .unwrap_or_else(|_| shell_panic(
                ShellError::ParsingError,
                format!("Cannot parse Token::Number({:?}) into a floating-point number.", number)
            ));
        
        return Statement::Number(result);
    }

    /// Pops the last [`Statement`] within the `statements` field, panicking if no [`Statement`]s are contained within the field.
    #[inline]
    fn pop_statement(&mut self) -> Statement {
        return self.statements.pop().unwrap();
    }
}


impl Parser {
    /// Constructs a new [`Parser`] out of a [`Lexer`].
    #[inline]
    pub const fn new(lexer: Lexer) -> Self {
        return Self {
            statements: Vec::new(),
            lexer,
        };
    }


    /// Returns a clone of the `statements` field in the [`Parser`], should be used after calling the [`parse`] method.
    #[inline]
    pub fn get_statements(&self) -> Vec<Statement> {
        return self.statements.clone();
    }


    /// Parses the [`Token`]s within the [`Lexer`] into [`Statement`]s.
    pub fn parse(&mut self) {
        while let Some(current_token) = self.lexer.next() {
            let s = match current_token {
                Token::Number(number) => {
                    let a = Self::parse_number(number);

                    match self.lexer.peek_token().unwrap_or(Token::Empty) {
                        // Attempts to create an Arithmetic Statement.
                        t if t.clone().is_arithmetic_operator() => {
                            let operator = self.lexer.next().unwrap();

                            // Attempt to retrieve a Token after the operator.
                            let b = self.lexer.get_token()
                                .unwrap_or_else(|| shell_panic(
                                    ShellError::SyntaxError,
                                    format!("Expected a number after Token::{:?}.", operator)
                                ));
                            
                            if let Token::Number(b) = b {
                                // Attempts to verify the operator as a valid BinaryOperation.
                                let operator = operator.clone().try_into()
                                    .unwrap_or_else(|err| shell_panic(
                                        err,
                                        format!("Cannot convert Token::{:?} into a BinaryOperation.", operator)
                                    ));
                                
                                let b = Self::parse_number(b);
                                Statement::Arithmetic(Box::new(a), operator, Box::new(b))
                            }

                            else {
                                shell_panic(
                                    ShellError::SyntaxError,
                                    format!("Expected a number after Token::{:?}.", operator)
                                )
                            }
                        },
                        
                        _ => a,
                    }
                }
                
                Token::Word(word) => {
                    Statement::Command(word.to_lowercase())
                },

                t if t.clone().is_arithmetic_operator() => {
                    let maybe_arithmetic = self.pop_statement();

                    if let Statement::Arithmetic(_, _, _) = maybe_arithmetic {
                        let operator = t.clone().try_into()
                            .unwrap_or_else(|err| shell_panic(
                                err,
                                format!("Cannot convert Token::{:?} into a BinaryOperation", t)
                            ));
                        
                        if let Some(Token::Number(b)) = self.lexer.next() {
                            let b = Self::parse_number(b);
                            Statement::Arithmetic(Box::new(maybe_arithmetic), operator, Box::new(b))
                        }
                        
                        else {
                            shell_panic(
                                ShellError::SyntaxError,
                                format!("Expected a number after Token::{:?}.", operator)
                            );
                        }
                    }

                    else {
                        shell_panic(
                            ShellError::EvaluationError,
                            format!("Expected an Arithmetic Statement before Token::{:?}.", t)
                        );
                    }
                }
                
                t => shell_panic(
                    ShellError::ParsingError,
                    format!("Cannot parse Token::{:?} into a Statement.", t)
                ),
            };

            self.statements.push(s);
        }
    }
}