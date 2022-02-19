/// [`Token`]s to be parsed after lexical analysis.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// [`Token`] containing a [`String`] of alphabetic characters.
    Word(String),

    /// [`Token`] containing a [`String`] that can be parsed into an integer or a floating-point number.
    Number(String),

    /// An empty [`Token`] used for defaults in unwrapping mainly.
    Empty,

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