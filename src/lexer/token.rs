/// [`Token`]s to be parsed after lexical analysis.
#[derive(Debug, PartialEq)]
pub enum Token {
    /// [`Token`] containing a [`String`] alphabetic characters.
    Word(String),
}