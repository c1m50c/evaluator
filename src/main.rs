pub mod parser;
pub mod lexer;
pub mod shell;
pub mod eval;


#[allow(unused_imports)]
use term_painter::{ToStyle, Color::*};
use std::thread;


fn main() {
    loop {
        let input = shell::input();

        let execution_thread = thread::spawn(move || {
            let lexer = lexer::Lexer::new(input);
            for t in lexer.clone() {
                shell::debug_print(
                    "Lexical Token", format!("{:?}", t).as_str()
                );
            }

            let mut parser = parser::Parser::new(lexer); 
            parser.parse();

            let statements = parser.get_statements();
            for s in statements.clone() {
                shell::debug_print(
                    "Parsing Statement", format!("{:?}", s).as_str()
                );
            }

            let mut evaluator = eval::Evaluator::new(statements);
            evaluator.evaluate();
        });

        let _ = execution_thread.join();
    }
}