mod parser;
mod lexer;
mod shell;


use term_painter::{ToStyle, Color::*};
use shell::input;
use lexer::Lexer;


fn main() {
    loop {
        let inp: String = input();
        if inp.is_empty() { break; }

        let mut lexer: Lexer = Lexer::new(inp.clone());

        if cfg!(debug_assertions) {
            while let Some(t) = lexer.next() {
                println!("{} {}: {}",
                    Green.bold().paint("[ DEBUG ]"),
                    Cyan.paint("Lexical Token"),
                    t
                );
            }
        }
    }
}