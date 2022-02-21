pub mod enums;

use super::shell::{ShellError, shell_panic};
use super::lexer::{token::Token, Lexer};
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
    /// Constructs a new [`Parser`] out of a [`Lexer`].
    #[inline]
    pub const fn new(lexer: Lexer) -> Self {
        return Self {
            statements: Vec::new(),
            lexer,
        };
    }


    /// Parses the [`Token`]s within the [`Lexer`] into [`Statement`]s.
    pub fn parse(&mut self) {
        while let Some(current_token) = self.lexer.next() {
            let s = match current_token {
                Token::Word(word) => {
                    Statement::Command(word.to_lowercase())
                },
                
                t => shell_panic(
                    ShellError::ParsingError,
                    format!("Cannot parse Token::{:?} into a Statement.", t)
                ),
            };

            self.statements.push(s);
        }
    }
}