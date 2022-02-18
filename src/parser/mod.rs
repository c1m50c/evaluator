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
    /// Parses the [`Token`] into a [`Statement`].
    fn parse_token(token: Token) -> Statement {
        return match token {
            Token::Word(s) => {
                Statement::Command(s)
            },

            Token::Number(n) => {
                // TODO: Allow for parsining of an Arithmetic Statement.
                
                let n = n.parse::<f64>()
                    .unwrap_or_else(|_| shell_panic(
                        ShellError::ParsingError,
                        format!("Cannot parse Token::Number({:?}) into a floating-point number.", n)
                    ));
                
                Statement::Number(n)
            }

            t => shell_panic(
                ShellError::ParsingError,
                format!("Cannot parse Token::{:?}.", t),
            ),
        };
    }
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
    pub fn parse(self) -> Vec<Statement> {
        let mut data = Vec::new();
        
        for token in self.lexer {
            data.push(Self::parse_token(token));
        }

        return data;
    }
}