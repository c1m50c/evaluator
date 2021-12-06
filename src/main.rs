mod shell;


use shell::input;


fn main() {
    loop {
        let inp: String = input();
        if inp.is_empty() { break; }
    }
}