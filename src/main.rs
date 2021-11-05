use term_painter::{Color::*, ToStyle};
use std::io::{self, Write};

mod tokenizer;


/// Gets user input from the terminal, prompt hardcoded.
fn get_input() -> String {
    let mut input: String = String::new();

    /* Prompt */
    print!("{}{}",
        BrightMagenta.bold().paint("evalulator"),
        BrightYellow.bold().paint(":$ "),
    );

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line.");
    return input.trim().to_string();
}


fn main() {
    loop {
        let inp = get_input();
        if inp.is_empty() { return; }

        let mut tokenizer = tokenizer::Tokenizer::new(inp);
    }
}
