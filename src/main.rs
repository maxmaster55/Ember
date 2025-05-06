use std::io::{self, BufReader};
use std::env;

mod token;
mod lexer;
mod repl;
mod parser;
mod ast;
mod object;
mod evaluator;

use repl::start;

fn main() {

    // get the args from the command line
    let args: Vec<String> = env::args().collect();

    // check if the args is empty
    if args.len() == 1 {
        let stdin = io::stdin();
        // to add "read_line" for the stdin
        let reader = BufReader::new(stdin.lock()); 
    
        let stdout = io::stdout();
        // lock to protect 
        let writer = stdout.lock();
        start(reader, writer, false);
        return;
    }

    let filename = &args[1];
    println!("Running file: {}", filename);
    let rfile = std::fs::File::open(filename).expect("Error opening file");
    let reader = BufReader::new(rfile);

    let wfile = std::fs::File::create("./o.par").expect("Error creating file");
    
    start(reader, wfile, true);


}
