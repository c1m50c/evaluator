pub(crate) mod enums;


use enums::Statement;
use super::lexer::{token::Token, Lexer};
use std::vec::Vec;


pub type ParsedData = Vec<Statement>;


pub struct Parser {
    lexer: Lexer,
}


impl Parser {
    #[inline(always)]
    pub const fn new(lexer: Lexer) -> Self {
        return Self {
            lexer,
        };
    }

    pub fn parse(&mut self) -> ParsedData {
        let mut tokens: Vec<Token> = Vec::new();
        let mut result: ParsedData = ParsedData::new();

        while let Some(t) = self.lexer.next() {
            tokens.push(t);
        }

        return result;
    }
}


fn get_binding_power(token: Token) -> Option<(usize, usize)> {
    return match token {
        Token::Caret => Some((8, 9)),
        Token::Star => Some((6, 7)),
        Token::ForwardSlash => Some((4, 5)),
        Token::Plus => Some((2, 3)),
        Token::Minus => Some((0, 1)),
        _ => None,
    };
}