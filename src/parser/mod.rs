pub mod enums;

use enums::*;
use super::shell::{ShellError, shell_panic};
use super::lexer::{Lexer, token::Token};
use std::vec::Vec;


#[derive(Debug, Clone)]
pub struct Parser {
    lexer: Lexer,
}


impl Parser {
    /// Constructs a new [`Parser`] with a [`Lexer`].
    #[inline]
    pub const fn new(lexer: Lexer) -> Self {
        return Self {
            lexer,
        };
    }

    /// Parses the [`Token`]s of the [`Lexer`] into [`Statement`]s to be evaluated.
    #[inline]
    pub fn parse(&mut self) -> Vec<Statement> {
        let mut data = Vec::new();
        
        while let Some(t) = self.lexer.get_token() {
            let token = match t {
                Token::Word(s) => {
                    Statement::Command(s)
                },
    
                Token::Number(n) => {
                    let a = n.parse::<f64>()
                        .unwrap_or_else(|_| shell_panic(
                            ShellError::ParsingError,
                            format!("Cannot parse Token::Number({:?}) into a floating-point number.", n)
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
                                    ShellError::ParsingError,
                                    format!("Expected a number after Token::{:?}.", operator)
                                ));
                            
                            if let Token::Number(b) = b {
                                let b = b.parse::<f64>()
                                    .unwrap_or_else(|_| shell_panic(
                                        ShellError::ParsingError,
                                        format!("Cannot parse Token::Number({:?}) into a floating-point number.", b)
                                    ));

                                let a = Statement::Number(a);
                                let b = Statement::Number(b);

                                Statement::Arithmetic(Box::new(a), operator, Box::new(b))
                            }
                            
                            else {
                                shell_panic(
                                    ShellError::SyntaxError,
                                    format!("Expected a number after Token::{:?}.", operator)
                                );
                            }
                        },

                        _ => Statement::Number(a),
                    }
                }
    
                t => shell_panic(
                    ShellError::ParsingError,
                    format!("Cannot parse Token::{:?}.", t),
                ),
            };

            data.push(token);
        }

        return data;
    }
}