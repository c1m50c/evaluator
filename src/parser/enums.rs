use std::boxed::Box;
use std::fmt;


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Statement {
    Arithmetic(Expression), // Arithmetic(ArithmeticExpression)
    Command(String), // Command(String)
}


#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum ArithmeticOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    Pow,
}


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expression {
    /* Mathematical */
    Arithmetic(Box<Expression>, ArithmeticOperation, Box<Expression>), // Arithmetic(Box<Float>, ArithmeticOperation, Box<Float>)
    
    /* Types */
    Float(f64), // Float(f64)
    String(String), // String(String)
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