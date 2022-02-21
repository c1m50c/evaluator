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

            let mut parser = parser::Parser::new(lexer.clone());
            
            if cfg!(debug_assertions) {
                for token in lexer {
                    shell::debug_print(
                        "Lexical Analysis",
                        format!("{:?}", token).as_str()
                    );
                }

                // for statement in parsed_data.clone() {
                //     shell::debug_print(
                //         "Parsing",
                //         format!("{:?}", statement).as_str()
                //     );
                // }
            }
        });

        let _ = execution_thread.join();
    }
}