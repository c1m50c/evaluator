/*
    TODO: Replace all panics with a function that kills the current shell iteration with an error.
        Could possibly do this by creating a thread for the execution of the current input, and killing it with a 
        function that mimics the panic! macro. This is so the user does not have to restart the shell every error.
*/

mod parser;
mod lexer;
mod shell;


use term_painter::{ToStyle, Color::*};
use shell::{input, evalulate};
use parser::Parser;
use lexer::Lexer;
use std::thread;


fn main() {
    loop {
        let inp: String = input();
        if inp.is_empty() { break; }

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

            for s in parsed_data {
                evalulate(s);
            }
        });

        let _ = execution_thread.join();
    }
}