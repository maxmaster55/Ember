use std::io::{BufRead, Write};

use crate::{evaluator, lexer::Lexer, object::Object, parser::Parser};



const PROMPT: &str = "=>";


// any type that implements the read trait
pub fn start<R: BufRead, W: Write>(mut reader: R, mut writer: W, is_file: bool) {
    loop {
        if !is_file{
            write!(writer, "{} ", PROMPT).expect("Error with the writer");
        }

        let _ = writer.flush();

        let mut written = String::new();
        reader.read_line(&mut written).expect("Error reading input!");

        if written.trim().is_empty() {
            continue; // Skip empty input
        }

        let lexer = Lexer::new(written);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();

        for stmnt in program.statements {
            writeln!(writer, "{:?}", evaluator::eval(stmnt).to_string()).expect("Error writing output");
        }



        if is_file {
            break;
        }       

    }
}