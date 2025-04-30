use std::io::{self, BufReader};

mod token;
mod lexer;
mod repl;
mod parser;
mod ast;

use repl::start;

fn main() {
    let stdin = io::stdin();
    // to add "read_line" for the stdin
    let reader = BufReader::new(stdin.lock()); 

    let stdout = io::stdout();
    // lock to protect 
    let writer = stdout.lock();
    start(reader, writer);

}
