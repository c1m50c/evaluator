/*
    TODO: Replace all panics with a function that kills the current shell iteration with an error.
*/

mod parser;
mod lexer;
mod shell;


use term_painter::{ToStyle, Color::*};
use shell::{input, evalulate};
use parser::Parser;
use lexer::Lexer;


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

            for s in parsed_data.clone() {
                println!("{} {}: {}",
                    Green.bold().paint("[ DEBUG ]"),
                    Cyan.paint("Parsing Statement"),
                    s
                );
            }
        }

        for s in parsed_data {
            evalulate(s);
        }
    }
}