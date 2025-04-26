use std::io::{BufRead, Write};

use crate::{lexer::Lexer, token::{Token, TokenType}};



const PROMPT: &str = "=>";


// any type that implements the read trait
pub fn start<R: BufRead, W: Write>(mut reader: R, mut writer: W) {
    loop {
        write!(writer, "{} ", PROMPT).expect("Error with the writer");
        writer.flush();

        let mut written = String::new();
        reader.read_line(&mut written).expect("Error reading input!");

        write!(writer, "in {}", written).expect("Error with the writer");
        writer.flush();


        let mut l = Lexer::new(written);
        
        let mut tok = l.next_token();

        while tok.t != TokenType::EOF {
            println!("{:?}", tok);
            tok = l.next_token();
        }
    }
}