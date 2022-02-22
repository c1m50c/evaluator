use super::super::lexer::token::Token;
use super::super::shell::ShellError;
use core::convert::TryFrom;
use std::boxed::Box;


/// [`Statement`]s to evaluated after being parsed.
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    /// `Command` [`Statement`] describing a `Command` that can be executed during evaluation,
    /// the [`String`] contains lowercase alphabetic characters.
    Command(String),

    /// `Number` [`Statement`] describing a floating-point `Number`.
    Number(f64),

    /// An `Arithmetic` [`Statement`] holds two `Number` or `Arithmetic` [`Statements`] and a [`BinaryOperation`],
    /// which together describe an arithmetic expression.
    Arithmetic(Box<Statement>, BinaryOperation, Box<Statement>),
}


/// Enum describing various [`BinaryOperation`]s.
#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperation {
    /// `+` Operator, `std::ops::Add`.
    Addition,

    /// `-` Operator, `std::ops::Sub`.
    Subtraction,

    /// `*` Operator, `std::ops::Mul`.
    Multiplication,

    /// `/` Operator, `std::ops::Div`.
    Division,

    /// `^` Operator, `f64.powf()`.
    Power,
}


impl TryFrom<Token> for BinaryOperation {
    type Error = ShellError;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::Plus => Ok(BinaryOperation::Addition),
            Token::Minus => Ok(BinaryOperation::Subtraction),
            Token::Star => Ok(BinaryOperation::Multiplication),
            Token::ForwardSlash => Ok(BinaryOperation::Division),
            Token::Caret => Ok(BinaryOperation::Power),

            _ => Err(ShellError::ParsingError),
        }
    }
}