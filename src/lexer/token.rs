/// [`Token`]s to be parsed after lexical analysis.
#[derive(Debug, PartialEq)]
pub enum Token {
    /// [`Token`] containing a [`String`] of alphabetic characters.
    Word(String),

    /// [`Token`] containing a [`String`] that can be parsed into an integer or a floating-point number.
    Number(String),

    /// `+` Operator.
    Plus,

    /// `-` Operator.
    Minus,

    /// `*` Operator.
    Star,

    /// `/` Operator.
    ForwardSlash,

    /// `=` Operator.
    Equal,

    /// `>` Operator.
    Greater,

    /// `<` Operator.
    Less,

    /// `^` Operator.
    Caret,
}