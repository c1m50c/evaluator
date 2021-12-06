/*
    TODO: Replace all panics with a function that kills the current shell iteration with an error.
*/
pub(crate) mod token;


use token::Token;
use std::{vec::Vec, option::Option};


pub struct Lexer {
    characters: Vec<char>,
    position: usize,
    next_position: usize,
    current_char: char,
}


/* Private Methods */
impl Lexer {
    /// Reads the next `char` in the `Lexer`'s `characters` Vector.
    #[inline]
    fn read(&mut self) {
        if self.next_position >= self.characters.len() {
            self.current_char = '\0';
        } else {
            self.current_char = self.characters[self.next_position];
        }

        self.position = self.next_position;
        self.next_position = self.position + 1;
    }

    /// Skips every whitespace `char` until it comes across a non-whitespace `char`.
    #[inline(always)]
    fn skip_whitespace(&mut self) {
        while self.current_char.is_whitespace() {
            self.read();
        }
    }

    /// Matches the `current_char` and possibly subsequent characters against various patterns to determine a `Token`.
    fn match_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.current_char {
            /* Seperators */
            '(' => Token::LeftParen,
            ')' => Token::RightParen,

            /* Operators */
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::ForwardSlash,

            /* Types */
            _ if self.current_char.is_numeric() => {
                /* Numbers */
                let mut string = String::new();
                string.push(self.current_char);
                self.read();

                while self.position < self.characters.len() && (self.current_char.is_numeric() || self.current_char == '.') {
                    string.push(self.current_char);
                    self.read();
                }

                if string.matches(".").count() <= 1 { Token::Number(string) }
                else { panic!("Too many decimals within the Number '{}'.", string); }
            }

            _ => panic!("Unimplemented Token '{}'.", self.current_char),
        };

        self.read();
        return token;
    }
}


/* Public Methods */
impl Lexer {
    #[inline(always)]
    pub fn new(input: String) -> Self {
        let mut result = Self {
            characters: input.chars().collect(),
            position: 0,
            next_position: 1,
            current_char: '\0',
        };

        result.current_char = result.characters[result.position];
        return result;
    }

    /// Peeks ahead to the next `Token`, while not incrementing the `Lexer`'s positional values.
    #[inline]
    pub fn peek(&mut self) -> Option<Token> {
        if self.next_position >= self.characters.len() { return None; }

        let old_position = self.position;
        let old_next_position = self.next_position;
        let old_current_char = self.current_char;
        
        self.current_char = self.characters[self.position];

        let token = self.match_token();

        self.position = old_position;
        self.next_position = old_next_position;
        self.current_char = old_current_char;

        return Some(token);
    }

    /// Resets the `Lexer` to its beginning state.
    #[inline]
    pub fn reset(&mut self) {
        self.position = 0;
        self.next_position = 1;
        self.current_char = self.characters[0];
    }
}


impl Iterator for Lexer {
    type Item = Token;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.next_position >= self.characters.len() { return None; }
        return Some(self.match_token());
    }
}