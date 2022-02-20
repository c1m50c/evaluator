pub mod enums;

use enums::*;
use super::shell::{ShellError, shell_panic};
use super::lexer::{Lexer, token::Token};
use std::vec::Vec;


#[derive(Debug, Clone)]
pub struct Parser {
    statements: Vec<Statement>,
    lexer: Lexer,
}


impl Parser {
    /// Creates a `Number` [`Statement`] out of a [`String`] from a `Number` [`Token`].
    #[inline]
    fn create_number_statement(number: String) -> Result<Statement, String> {
        return match number.parse::<f64>() {
            Ok(n) => Ok(Statement::Number(n)),
            Err(_) => Err(format!("Cannot parse Token::Number({:?}) into a floating-point number.", number)),
        }
    }
}


impl Parser {
    /// Constructs a new [`Parser`] with a [`Lexer`].
    #[inline]
    pub const fn new(lexer: Lexer) -> Self {
        return Self {
            statements: Vec::new(),
            lexer,
        };
    }

    #[inline]
    pub fn previous_statement(&self) -> &Statement {
        return &self.statements[self.statements.len() - 1];
    }

    #[inline]
    pub fn pop_previous_statement(&mut self) -> Statement {
        return self.statements.pop().unwrap_or_else(|| shell_panic(
            ShellError::ParsingError,
            "Cannot pop the previous Statement from the Parser."
        ));
    }

    /// Parses the [`Token`]s of the [`Lexer`] into [`Statement`]s to be evaluated.
    #[inline]
    pub fn parse(&mut self) -> Vec<Statement> {
        while let Some(t) = self.lexer.get_token() {
            let token = match t {
                Token::Word(s) => {
                    if let Some(Token::Number(n)) = self.lexer.peek_token() {
                        self.lexer.skip_token();

                        let n = Self::create_number_statement(n)
                            .unwrap_or_else(|err| shell_panic(
                                ShellError::ParsingError, err
                            ));
                        
                        Statement::MathematicalFunction(s, Box::new(n))
                    }

                    else { Statement::Command(s) }
                },
    
                Token::Number(n) => {
                    let a = Self::create_number_statement(n)
                        .unwrap_or_else(|err| shell_panic(
                            ShellError::ParsingError, err
                        ));
                    
                    /* Parse into an Arithmetic Statement if possible. */
                    match self.lexer.peek_token().unwrap_or(Token::Empty) {
                        Token::Plus |
                        Token::Minus |
                        Token::Star |
                        Token::ForwardSlash |
                        Token::Caret => {
                            let operator = self.lexer.get_token().unwrap();

                            let b = self.lexer.get_token()
                                .unwrap_or_else(|| shell_panic(
                                    ShellError::SyntaxError,
                                    format!("Expected a number after Token::{:?}.", operator)
                                ));
                            
                            if let Token::Number(b) = b {
                                let b = Self::create_number_statement(b)
                                    .unwrap_or_else(|err| shell_panic(
                                        ShellError::ParsingError, err
                                    ));

                                Statement::Arithmetic(Box::new(a), operator, Box::new(b))
                            }
                            
                            else {
                                shell_panic(
                                    ShellError::SyntaxError,
                                    format!("Expected a number after Token::{:?}.", operator)
                                );
                            }
                        },

                        _ => a,
                    }
                },

                Token::Plus |
                Token::Minus |
                Token::Star |
                Token::ForwardSlash |
                Token::Caret => {
                    match self.previous_statement() {
                        Statement::Arithmetic(_, _, _) => {
                            let a = self.pop_previous_statement();
                            let operator = t;

                            let b = self.lexer.get_token()
                                .unwrap_or_else(|| shell_panic(
                                    ShellError::SyntaxError,
                                    format!("Expected a number after Token::{:?}.", operator)
                                ));
                            
                            if let Token::Number(b) = b {
                                let b = Self::create_number_statement(b)
                                    .unwrap_or_else(|err| shell_panic(
                                        ShellError::ParsingError, err
                                    ));
                                
                                Statement::Arithmetic(Box::new(a), operator, Box::new(b))
                            }

                            else {
                                shell_panic(
                                    ShellError::SyntaxError,
                                    format!("Expected a number after Token::{:?}.", operator)
                                );
                            }
                        },

                        _ => shell_panic(
                            ShellError::ParsingError,
                            format!("No Arithmetic Statement prior to Token::{:?}.", t)
                        ),
                    }
                },
    
                t => shell_panic(
                    ShellError::ParsingError,
                    format!("Cannot parse Token::{:?}.", t),
                ),
            };

            self.statements.push(token);
        }

        return self.statements.clone();
    }
}