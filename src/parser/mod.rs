pub(crate) mod enums;


use enums::*;
use super::lexer::{token::Token, Lexer};
use super::shell::print_debug;
use super::shell::error::*;
use std::vec::Vec;


pub type ParsedData = Vec<Statement>;


pub struct Parser {
    lexer: Lexer,
}


impl Parser {
    fn parse_expression(&mut self, binding_power: usize) -> Expression {
        let mut left = match self.lexer.next() {
            Some(Token::Word(x)) => {
                Expression::String(x)
            },

            Some(Token::Number(x)) => {
                let converted = shell_expect_ok(
                    x.parse::<f64>(),
                    format!("Cannot convert '{}' to a 64-bit floating point number.", x).as_ref(),
                    ShellError::ParsingError
                );

                Expression::Float(converted)
            },
            
            _ => shell_panic(
                "Cannot evaluate expression.",
                ShellError::ParsingError
            ),
        };

        loop {
            let operator = match self.lexer.peek() {
                Some(t) => t,
                None => break,
            };

            if let Some((left_binding_power, right_binding_power)) = get_binding_power(operator) {
                if left_binding_power < binding_power { break; }

                let next_operator = shell_expect_some(
                    self.lexer.next(),
                    "Cannot retrieve the next operator within the Parser's Lexer.",
                    ShellError::ParsingError
                );

                let right = self.parse_expression(right_binding_power);
                left = make_arithmetic_expression(Box::new(left), next_operator, Box::new(right));

                continue;
            }

            break;
        }
        
        return left;
    }
}


impl Parser {
    #[inline(always)]
    pub const fn new(lexer: Lexer) -> Self {
        return Self {
            lexer,
        };
    }

    pub fn parse(&mut self) -> ParsedData {
        let mut result: ParsedData = ParsedData::new();

        while let Some(t) = self.lexer.next() {
            print_debug("Parsing Token", format!("{}", t).as_ref());

            match t {
                Token::Word(x) => {
                    result.push(Statement::Command(x));
                },
                
                Token::Number(_) => {
                    self.lexer.back(); // TODO: This is not properly reversing
                    let expression = self.parse_expression(0);
                    result.push(Statement::Arithmetic(expression));
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


fn make_arithmetic_expression(left: Box<Expression>, operator: Token, right: Box<Expression>) -> Expression {
    match operator {
        Token::Plus => Expression::Arithmetic(left, ArithmeticOperation::Addition, right),
        Token::Minus => Expression::Arithmetic(left, ArithmeticOperation::Subtraction, right),
        Token::Star => Expression::Arithmetic(left, ArithmeticOperation::Multiplication, right),
        Token::ForwardSlash => Expression::Arithmetic(left, ArithmeticOperation::Division, right),
        Token::Percent => Expression::Arithmetic(left, ArithmeticOperation::Modulo, right),
        Token::Caret => Expression::Arithmetic(left, ArithmeticOperation::Pow, right),

        _ => shell_panic(
            format!("Cannot create expression from '{}, {}, {}'.", left, operator, right).as_ref(),
            ShellError::ParsingError
        ),
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