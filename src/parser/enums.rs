use super::super::lexer::token::Token;
use std::boxed::Box;


#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    /// An arithmetic statement, containing `Statement::(Arithmetic || Number)`, `Token::(Arithmetic Operator)` and a `Statement::(Arithmetic || Number)`.
    Arithmetic(Box<Statement>, Token, Box<Statement>),
    
    /// A statement containing an alphabetic [`String`].
    Command(String),

    /// A statement containing a [`f64`] number.
    Number(f64),
}