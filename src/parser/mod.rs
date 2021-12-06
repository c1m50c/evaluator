use super::lexer::token::Token;


fn get_binding_power(token: Token) -> Option<(usize, usize)> {
    return match token {
        Token::Caret => Some((8, 9)),
        Token::Star => Some((6, 7)),
        Token::ForwardSlash => Some((4, 5)),
        Token::Plus => Some((2, 3)),
        Token::Minus => Some((0, 1)),
        _ => None,
    };
}