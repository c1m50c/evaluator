/// [`Statement`]s to evaluated after being parsed.
pub enum Statement {
    /// `Command` [`Statement`] describing a `Command` that can be executed during evaluation,
    /// the [`String`] contains lowercase alphabetic characters.
    Command(String),
}