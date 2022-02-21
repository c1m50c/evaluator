use super::parser::Parser;


pub struct Evaluator {
    parser: Parser,
}


impl Evaluator {
    #[inline]
    pub const fn new(parser: Parser) -> Self {
        return Self {
            parser,
        };
    }
}