pub mod lexer;
pub mod shell;


#[allow(unused_imports)]
use term_painter::{ToStyle, Color::*};
use std::thread;


fn main() {
    loop {
        let inp = shell::input();

        let execution_thread = thread::spawn(move || {
            let lexer = lexer::Lexer::new(inp);
            
            if cfg!(debug_assertions) {
                for token in lexer {
                    shell::debug_print(
                        "Lexical Analyis",
                        format!("{:?}", token).as_str()
                    );
                }
            }
        });

        let _ = execution_thread.join();
    }
}