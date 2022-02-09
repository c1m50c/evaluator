/// [`Token`]s to be parsed after lexical analysis.
#[derive(Debug, PartialEq)]
pub enum Token {
    /// [`Token`] containing a [`String`] alphabetic characters.
    Word(String),

    /// [`Token`] containg a [`String`] than can be parsed into a integer or a floating-point number.
    Number(String),
}