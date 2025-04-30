use std::io::{BufRead, Write};

use crate::{lexer::Lexer, parser::Parser};



const PROMPT: &str = "=>";


// any type that implements the read trait
pub fn start<R: BufRead, W: Write>(mut reader: R, mut writer: W) {
    loop {
        write!(writer, "{} ", PROMPT).expect("Error with the writer");
        let _ = writer.flush();

        let mut written = String::new();
        reader.read_line(&mut written).expect("Error reading input!");

        if written.trim().is_empty() {
            continue; // Skip empty input
        }

        let lexer = Lexer::new(written);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();

        for statement in program.statements {
            writeln!(writer, "{:?}", statement).expect("Error writing output");
        }
    }
}