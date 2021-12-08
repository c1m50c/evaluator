use std::fmt;


#[derive(Debug)]
pub enum Token {
    /* Types */
    Number(String),
    Word(String),

    /* Seperators */
    LeftParen,
    RightParen,

    /* Operators */
    Plus,
    Minus,
    Star,
    ForwardSlash,
    Caret,
}


impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{:?}", self);
    }
}