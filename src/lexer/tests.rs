use super::{Lexer, token::Token};


#[test]
fn token_word() {
    let mut lexer = Lexer::new("Hello World".to_string());

    assert_eq!(lexer.next(), Some(Token::Word("Hello".to_string())));
    assert_eq!(lexer.next(), Some(Token::Word("World".to_string())));
    assert_eq!(lexer.next(), None);
}


#[test]
fn token_number() {
    let mut lexer = Lexer::new("500 5.00".to_string());

    assert_eq!(lexer.next(), Some(Token::Number("500".to_string())));
    assert_eq!(lexer.next(), Some(Token::Number("5.00".to_string())));
    assert_eq!(lexer.next(), None);
}


#[test]
fn token_operators() {
    let mut lexer = Lexer::new("+ - * / = < > ^".to_string());

    assert_eq!(lexer.next(), Some(Token::Plus));
    assert_eq!(lexer.next(), Some(Token::Minus));
    assert_eq!(lexer.next(), Some(Token::Star));
    assert_eq!(lexer.next(), Some(Token::ForwardSlash));
    assert_eq!(lexer.next(), Some(Token::Equal));
    assert_eq!(lexer.next(), Some(Token::Greater));
    assert_eq!(lexer.next(), Some(Token::Less));
    assert_eq!(lexer.next(), Some(Token::Caret));
    assert_eq!(lexer.next(), None);
}