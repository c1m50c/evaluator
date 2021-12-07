use std::boxed::Box;
use std::fmt;


#[derive(Debug, Clone)]
pub enum Statement {
    Arithmetic(Expression), // Arithmetic(ArithmeticExpression)
    Command(String), // Command(String)
}


#[derive(Debug, Clone)]
pub enum ArithmeticOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}


#[derive(Debug, Clone)]
pub enum Expression {
    /* Mathematical */
    Arithmetic(Box<Expression>, ArithmeticOperation, Box<Expression>),
    
    /* Types */
    Float(f64),
}


impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{:?}", self);
    }
}


impl fmt::Display for ArithmeticOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{:?}", self);
    }
}


impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{:?}", self);
    }
}