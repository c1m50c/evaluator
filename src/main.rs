mod lexer;
mod shell;


use shell::input;
use lexer::Lexer;


fn main() {
    loop {
        let inp: String = input();
        if inp.is_empty() { break; }
        
        let mut lexer: Lexer = Lexer::new(inp.clone());
    }
}