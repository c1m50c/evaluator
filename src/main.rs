mod parser;
mod lexer;
mod shell;


use term_painter::{ToStyle, Color::*};
use shell::eval::evaluate_statements;
use parser::Parser;
use lexer::Lexer;
use shell::input;
use std::thread;


fn main() {
    loop {
        let inp: String = input();
        if inp.is_empty() { continue; }

        let execution_thread = thread::spawn(move || {
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

            evaluate_statements(parsed_data);
        });

        let _ = execution_thread.join();
    }
}