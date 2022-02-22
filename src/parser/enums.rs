/// [`Statement`]s to evaluated after being parsed.
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    /// `Command` [`Statement`] describing a `Command` that can be executed during evaluation,
    /// the [`String`] contains lowercase alphabetic characters.
    Command(String),

    /// `Number` [`Statement`] describing a floating-point `Number`.
    Number(f64),
}