mod parser;
mod lexer;
mod shell;


#[allow(unused_imports)]
use term_painter::{ToStyle, Color::*};

use shell::eval::evaluate_statements;
use shell::{input, print_debug};
use parser::Parser;
use lexer::Lexer;

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
                    print_debug("Lexical Token", format!("{}", t).as_ref());
                }

                for s in parsed_data.clone() {
                    print_debug("Parsing Statement", format!("{}", s).as_ref());
                }
            }

            evaluate_statements(parsed_data);
        });

        let _ = execution_thread.join();
    }
}