pub(crate) mod enums;


use enums::*;
use super::lexer::{token::Token, Lexer};
use super::shell::error::*;
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

        while let Some(t) = tokens.pop() {
            match t {
                Token::Number(x) => {
                    // TODO: Support for multiple operations in same statement.
                    if let Ok(a) = x.parse::<f64>() {
                        let operator: ArithmeticOperation;

                        if let Some(o) = tokens.pop() {
                            match o {
                                Token::Plus => operator = ArithmeticOperation::Addition,
                                Token::Minus => operator = ArithmeticOperation::Subtraction,
                                Token::Star => operator = ArithmeticOperation::Multiplication,
                                Token::ForwardSlash => operator = ArithmeticOperation::Division,
                                Token::Percent => operator = ArithmeticOperation::Modulo,
                                Token::Caret => operator = ArithmeticOperation::Pow,

                                _ => shell_panic(
                                    format!("Token '{}' is not a valid operator.", o).as_ref(),
                                    ShellError::SyntaxError
                                ),
                            }
                        } else {
                            shell_panic(
                                "Cannot retrieve next token in statement",
                                ShellError::SyntaxError
                            );
                        }

                        if let Some(b) = tokens.pop() {
                            match b {
                                Token::Number(b) => {
                                    if let Ok(b) = b.parse::<f64>() {
                                        let expression = Expression::Arithmetic(
                                            Box::new(Expression::Float(a)),
                                            operator,
                                            Box::new(Expression::Float(b)),
                                        );
    
                                        result.push(Statement::Arithmetic(expression));
                                    } else {
                                        shell_panic(
                                            format!("Cannot convert '{}' to a 64-bit floating point number.", b).as_ref(),
                                            ShellError::ParsingError
                                        );
                                    }
                                }

                                _ => shell_panic(
                                    format!("Token '{}' is not a number.", b).as_ref(),
                                    ShellError::SyntaxError
                                ),
                            }
                        } else {
                            shell_panic(
                                "Cannot retrieve next token in statement",
                                ShellError::SyntaxError
                            );
                        }
                    } else {
                        shell_panic(
                            format!("Cannot convert '{}' to a 64-bit floating point number.", x).as_ref(),
                            ShellError::ParsingError
                        );
                    }
                },

                Token::Word(x) => {
                    result.push(Statement::Command(x));
                },

                _ => shell_panic(
                    format!("Cannot convert token '{}' into a statement.", t).as_ref(),
                    ShellError::ParsingError
                ),
            }
        }

        return result;
    }
}


fn get_binding_power(token: Token) -> Option<(usize, usize)> {
    return match token {
        Token::Caret => Some((8, 9)), // Exponent
        Token::Star => Some((6, 7)), // Multiplication
        Token::ForwardSlash | Token::Percent => Some((4, 5)), // Division | Modulo
        Token::Plus => Some((2, 3)), // Addition
        Token::Minus => Some((0, 1)), // Subtraction
        _ => None,
    };
}