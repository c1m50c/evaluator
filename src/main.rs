mod parser;
mod lexer;
mod shell;


use term_painter::{ToStyle, Color::*};
use parser::Parser;
use lexer::Lexer;
use shell::input;


fn main() {
    loop {
        let inp: String = input();
        if inp.is_empty() { break; }

        let mut lexer: Lexer = Lexer::new(inp.clone());
        let mut parser: Parser = Parser::new(lexer.clone());
        let parsed_data = parser.parse();

        if cfg!(debug_assertions) {
            while let Some(t) = lexer.next() {
                println!("{} {}: {}",
                    Green.bold().paint("[ DEBUG ]"),
                    Cyan.paint("Lexical Token"),
                    t
                );
            }

            for s in parsed_data {
                println!("{} {}: {}",
                    Green.bold().paint("[ DEBUG ]"),
                    Cyan.paint("Parsed Statement"),
                    s
                );
            }
        }
    }
}