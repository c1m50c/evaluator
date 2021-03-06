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


impl Token {
    /// Checks the [`Token`] determining if it is a valid operator.
    /// 
    /// # Example
    /// ```rust
    /// let a = Token::Equal; let b = Token::Number(String::from("5"));
    /// assert!(a.is_operator());
    /// assert!(!b.is_operator());
    /// ```
    #[inline]
    pub fn is_operator(self) -> bool {
        match self {
            Token::Plus |
            Token::Minus |
            Token::Star |
            Token::ForwardSlash |
            Token::Caret |
            Token::Equal |
            Token::Greater |
            Token::Less => true,
            
            _ => false,
        }
    }

    /// Checks the [`Token`] determining if it is a valid arithmetic operator.
    /// 
    /// # Example
    /// ```rust
    /// let a = Token::Plus; let b = Token::Equal;
    /// assert!(a.is_arithmetic_operator());
    /// assert!(!b.is_arithmetic_operator());
    /// ```
    #[inline]
    pub fn is_arithmetic_operator(self) -> bool {
        match self {
            Token::Plus |
            Token::Minus |
            Token::Star |
            Token::ForwardSlash |
            Token::Caret => true,
            
            _ => false,
        }
    }
}