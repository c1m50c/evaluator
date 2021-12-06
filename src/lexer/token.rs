use std::fmt;


#[derive(Debug)]
pub enum Token {
    /* Types */
    Number(String),

    /* Seperators */
    LeftParen,
    RightParen,

    /* Operators */
    Plus,
    Minus,
    Star,
    ForwardSlash,
}


impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{:?}", self);
    }
}