pub mod token;

use super::shell::{ShellError, shell_panic};
use core::iter::Iterator;
use std::vec::Vec;
use token::Token;


/// Struct for performing lexical analysis on a [`String`].
pub struct Lexer {
    /// [`char`]s of the passed [`String`] in the constructor.
    characters: Vec<char>,

    /// Current `position` within `characters`.
    position: usize,
}


impl Lexer {
    /// Skips all empty or whitespace [`char`]s.
    #[inline]
    fn skip_empty(&mut self) {
        while self.characters[self.position].is_whitespace() {
            self.position += 1;
        }
    }
}


impl Lexer {
    /// Constructs a new [`Lexer`] from a `String`.
    /// 
    /// ## Example
    /// ```rust
    /// let mut lexer = Lexer::new("Hello There".to_string());
    /// 
    /// assert_eq!(lexer.next(), Some(Token::Word("Hello".to_string())));
    /// assert_eq!(lexer.next(), Some(Token::Word("There".to_string())));
    /// assert_eq!(lexer.next(), None);
    /// ```
    #[inline]
    pub fn new(string: String) -> Self {
        return Self {
            characters: string.chars().collect(),
            position: 0,
        };
    }

    /// Returns the `len` of the `characters` field in the [`Lexer`].
    /// 
    /// ## Example
    /// ```rust
    /// let lexer = Lexer::new("Hello".to_string())
    /// assert_eq!(lexer.len(), 5);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        return self.characters.len();
    }

    /// Returns the [`char`] at the index of the [`Lexer`]'s `position` in its `characters`.
    /// 
    /// ## Example
    /// ```rust
    /// let lexer = Lexer::new("Yoyo".to_string());
    /// assert_eq!(lexer.current(), 'Y');
    /// ```
    #[inline]
    pub fn current(&self) -> char {
        return self.characters[self.position];
    }

    /// Resets the [`Lexer`]'s `position` to zero.
    /// 
    /// ## Example
    /// ```rust
    /// let mut lexer = Lexer::new("Two Words");
    /// let a = lexer.next();
    /// lexer.reset();
    /// let b = lexer.next();
    /// 
    /// assert_eq!(a, Some(Token::("Two".to_string())));
    /// assert_eq!(b, Some(Token::("Two".to_string())));
    /// ```
    #[inline]
    pub fn reset(&mut self) {
        self.position = 0;
    }

    /// Returns the next [`Token`] within the [`Lexer`], equivalent to the `next()` method.
    /// 
    /// ## Example
    /// ```rust
    /// let mut lexer = Lexer::new("WordA WordB".to_string());
    /// 
    /// assert_eq!(lexer.get_token(), Token::Word("WordA".to_string()));
    /// assert_eq!(lexer.get_token(), Token::Word("WordB".to_string()));
    /// assert_eq!(lexer.get_token(), None);
    /// ```
    #[inline]
    pub fn get_token(&mut self) -> Option<Token> {
        if self.position >= self.characters.len() {
            return None;
        }
        
        self.skip_empty();

        let token = match self.characters[self.position] {
            c if c.is_numeric() => {
                let mut string = String::new();
                string.push(c);
                self.position += 1;

                while self.position < self.len() && (self.current().is_numeric() || self.current() == '.') {
                    string.push(self.current());
                    self.position += 1;
                }

                if string.matches(".").count() > 1 {
                    shell_panic(
                        ShellError::SyntaxError,
                        format!("Too many decimals within the Token::Number(\"{}\").", string)
                    );
                }

                Token::Number(string)
            },

            c if c.is_alphabetic() => {
                let mut string = String::new();
                string.push(c);
                self.position += 1;

                while self.position < self.len() && self.current().is_alphabetic() {
                    string.push(self.current());
                    self.position += 1;
                }

                Token::Word(string)
            },

            c => shell_panic(
                ShellError::SyntaxError,
                format!("Character '{}' is not in a valid Token pattern.", c)
            ),
        };

        return Some(token);
    }
}


impl Iterator for Lexer {
    type Item = Token;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        return self.get_token();
    }
}