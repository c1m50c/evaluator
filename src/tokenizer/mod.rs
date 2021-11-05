pub(crate) mod token;
use std::vec::Vec;
use token::Token;


#[derive(Debug)]
pub struct Tokenizer {
    contents: Vec<char>,
    cur_char: char,
    cur_pos: usize,
    next_pos: usize,
}


/* Private Methods */
impl Tokenizer {
    #[inline]
    fn read(&mut self) {
        if self.next_pos >= self.contents.len() { self.cur_char = '\0'; }
        else { self.cur_char = self.contents[self.next_pos]; }

        self.cur_pos = self.next_pos;
        self.next_pos = self.cur_pos + 1;
    }

    #[inline]
    fn skip_whitespace(&mut self) {
        while self.cur_char.is_whitespace() {
            self.read();
        }
    }

    fn match_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.cur_char {
            /* Seperators */
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            
            /* Operators */
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::ForwardSlash,

            _ if self.cur_char.is_numeric() => {
                /* Numbers */
                let mut string = String::new();
                string.push(self.cur_char);
                self.read();

                while self.cur_pos < self.contents.len() && (self.cur_char.is_numeric() || self.cur_char == '.') {
                    string.push(self.cur_char);
                    self.read();
                }

                if string.matches(".").count() <= 1 { Token::Number(string) }
                else { panic!("Too many decimals in number '{}'.", string); }
            },

            _ => panic!("Unimplemented Token '{}'.", self.cur_char),
        };

        self.read();
        return token;
    }
}


/* Public Methods */
impl Tokenizer {
    #[inline]
    pub fn new(input: String) -> Self {
        let mut ret = Self {
            contents: input.chars().collect(),
            cur_pos: 0,
            next_pos: 1,
            cur_char: '\0',
        };

        ret.cur_char = ret.contents[ret.cur_pos];
        return ret;
    }

    #[inline]
    pub fn peek(&mut self) -> Option<Token> {
        if self.next_pos >= self.contents.len() { return None; }

        let old_cur_pos = self.cur_pos;
        let old_next_pos = self.next_pos;
        let old_cur_char = self.cur_char;
        self.cur_char = self.contents[self.cur_pos];

        let token = self.match_token();

        self.cur_pos = old_cur_pos;
        self.next_pos = old_next_pos;
        self.cur_char = old_cur_char;
        
        return Some(token);
    }
}